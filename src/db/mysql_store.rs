use log::error;
use std::time::Duration;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use teleport_server::{
    clip::{ClipMessage, ClipType, Ttl},
    util::numbers::only_one_one,
};

use super::{
    entity::{
        clip_record::{self, Entity as ClipRecordEntity, Model as ClipRecordModel},
        user::{self, Entity as UserEntity, Model as UserModel},
    },
    User,
};

pub struct Store {
    options: Option<ConnectOptions>,
    conn: Option<DatabaseConnection>,
}

impl Store {
    pub fn new(url: &str) -> Self {
        let mut uri = url;
        if url.len() == 0 {
            uri = "mysql://macoo:mq2020.@localhost:3306/teleport";
        }

        let mut opt = ConnectOptions::new(uri.to_string());
        opt.max_connections(32)
            .min_connections(4)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8));
        // .sqlx_logging(true)
        // .sqlx_logging_level(log::LevelFilter::Info);

        Store {
            options: Some(opt),
            conn: None,
        }
    }

    /// connect to mysql
    pub async fn connect(&mut self) {
        if let Some(opt) = &self.options {
            let conn = Database::connect(opt.to_owned()).await.unwrap();
            self.conn = Some(conn)
        }
    }

    /// add user to db
    pub async fn add_user(&self, user: User) -> Result<UserModel, String> {
        use sea_orm::ActiveValue::Set;

        if let Some(db) = &self.conn {
            let data_type = if user.clip_authorities.contains(&ClipType::All) {
                ClipType::All as u8
            } else {
                user.clip_authorities.into_iter().map(|tp| tp as u8).sum()
            };

            let new_user = user::ActiveModel {
                ttl: Set(user.ttl as u8),
                data_type: Set(data_type),
                user_key: Set(user.key),
                user_secret: Set(user.secret),
                description: Set(user.description),
                ..Default::default()
            };

            let result = new_user.insert(db).await;
            match result {
                Ok(model) => Ok(model),
                Err(err) => Err(format!("{:?}", err)),
            }
        } else {
            log::error!("Connection hasn't been established!");
            return Err("Connection hasn't been established!".to_string());
        }
    }

    pub async fn add_clip_record(
        &self,
        usr: &UserModel,
        clip: &ClipMessage,
    ) -> Result<clip_record::Model, String> {
        use sea_orm::ActiveValue::Set;

        if let Some(db) = &self.conn {
            let data_type: u8 = clip.data_type as u8;
            let allowed_data_type = usr.data_type;

            // check ttl authority
            if usr.ttl != Ttl::Permanent as u8 {
                return Err(String::from("No authority to perpetuate data."));
            }

            // check data type, only single data type is valid, such as 1, 2, 4, 8...
            if !only_one_one(data_type) {
                return Err(String::from(
                    "Invalid data_type, only single type specification is allowed.",
                ));
            }

            // check data type authority
            if allowed_data_type & data_type != data_type {
                return Err(format!(
                    "No authority to perpetuate this kind of data: {:?}.",
                    allowed_data_type
                ));
            }

            let result = clip_record::ActiveModel {
                user_id: Set(usr.id),
                data_type: Set(data_type),
                content: Set(clip.data.to_owned()),
                ..Default::default()
            }
            .insert(db)
            .await;

            match result {
                Ok(model) => Ok(model),
                Err(err) => Err(format!("{:?}", err)),
            }
        } else {
            log::error!("Connection hasn't been established!");
            return Err("Connection hasn't been established!".to_string());
        }
    }

    pub async fn find_user(&self, user_key: &str) -> Option<UserModel> {
        if let Some(db) = &self.conn {
            let result = UserEntity::find()
                .filter(user::Column::UserKey.eq(user_key))
                .one(db)
                .await;
            match result {
                Ok(usr) => usr,
                Err(err) => {
                    error!("Error while fetching user: {}", err);
                    None
                }
            }
        } else {
            log::error!("Connection hasn't been established!");
            return None;
        }
    }

    pub async fn find_clips(
        &self,
        usr: &UserModel,
        size: usize,
    ) -> Result<Vec<ClipRecordModel>, String> {
        if let Some(db) = &self.conn {
            let result = ClipRecordEntity::find()
                .filter(clip_record::Column::UserId.eq(usr.id))
                .order_by_desc(clip_record::Column::CreatedAt)
                .paginate(db, size)
                .fetch()
                .await;

            match result {
                Ok(models) => Ok(models),
                Err(err) => Err(format!("{:?}", err)),
            }
        } else {
            log::error!("Connection hasn't been established!");
            return Err("Connection hasn't been established!".to_string());
        }
    }
}
