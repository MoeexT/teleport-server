use std::collections::HashSet;


use teleport_server::{db::{mysql_store::{connect, find_user, add_user}}, ClipType, Ttl};



#[tokio::test]
async fn select() {
    let db = &connect().await;
    dbg!(find_user(db, "root_app_key").await);
}

#[tokio::test]
#[ignore = "Not always insert value to db"]
async fn insert() {
    let db = &connect().await;
    let mut set = HashSet::new();
    set.insert(ClipType::All);
    add_user(db, set, Ttl::Permanent).await.unwrap();
}
