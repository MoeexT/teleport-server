use std::collections::HashSet;

use teleport_server::{
    clip::{ClipType, Ttl},
    db::{mysql_store::Store, User},
};

#[tokio::test]
#[ignore = "Not always insert value to db"]
async fn insert() {
    let mut store = Store::new("");
    store.connect().await;
    let mut set = HashSet::new();
    set.insert(ClipType::All);
    let user = User::new("root", "root_secret", "administrator", Ttl::Transient, set);
    store.add_user(user).await.unwrap();
}

#[tokio::test]
async fn select() {
    let mut store = Store::new("");
    store.connect().await;
    dbg!(store.find_user("root_app_key").await);
}
