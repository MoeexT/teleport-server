use log::error;
use std::{collections::HashSet, time::Duration};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use super::entity::{
    clip_record::{self, Entity as ClipRecordEntity, Model as ClipRecordModel},
    user::{self, Entity as UserEntity, Model as UserModel},
};
use crate::{util::numbers::only_one_one, ClipType, EnumToNumber, Ttl};

/// connect to mysql
pub async fn connect() -> DatabaseConnection {
    let url = "mysql://macoo:mq2020.@localhost:3306/teleport";
    let mut opt = ConnectOptions::new(url.to_owned());
    opt.max_connections(32)
        .min_connections(4)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));
    // .sqlx_logging(true)
    // .sqlx_logging_level(log::LevelFilter::Info);

    Database::connect(opt).await.unwrap()
}

/// add user to db
pub async fn add_user(
    db: &DatabaseConnection,
    data_types: HashSet<ClipType>,
    ttl: Ttl,
) -> Result<UserModel, String> {
    use sea_orm::ActiveValue::Set;

    let data_type = if data_types.contains(&ClipType::All) {
        ClipType::All.convert_to_number::<u8>()
    } else {
        data_types
            .into_iter()
            .map(|tp| tp.convert_to_number::<u8>())
            .sum()
    };

    let new_user = user::ActiveModel {
        ttl: Set(ttl.convert_to_number::<u8>()),
        data_type: Set(data_type),
        user_key: Set("root_app_key".to_owned()),
        user_secret: Set("root_app_secret".to_owned()),
        description: Set("root user".to_owned()),
        ..Default::default()
    };

    let result = new_user.insert(db).await;
    match result {
        Ok(model) => Ok(model),
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub async fn add_clip_record(
    db: &DatabaseConnection,
    usr: &UserModel,
    data_type: u8,
    content: &[u8],
) -> Result<clip_record::Model, String> {
    use sea_orm::ActiveValue::Set;

    let data_type_authority = usr.data_type;

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
    if data_type_authority & data_type != data_type {
        return Err(format!(
            "No authority to perpetuate this kind of data: {:?}.",
            data_type_authority
        ));
    }

    let result = clip_record::ActiveModel {
        user_id: Set(usr.id),
        data_type: Set(data_type),
        content: Set(content.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await;

    match result {
        Ok(model) => Ok(model),
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub async fn find_user(db: &DatabaseConnection, key: &str) -> Option<UserModel> {
    let result = UserEntity::find()
        .filter(user::Column::UserKey.eq(key))
        .one(db)
        .await;
    match result {
        Ok(usr) => usr,
        Err(err) => {
            error!("Error while fetching user: {}", err);
            None
        }
    }
}

pub async fn find_clips(
    db: &DatabaseConnection,
    usr: &UserModel,
    size: usize,
) -> Result<Vec<ClipRecordModel>, String> {
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
}
