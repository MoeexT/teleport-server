use teleport_server::db::{mysql_store::Store, redis_store};

#[tokio::test]
#[ignore = "Not insert value to redis"]
async fn ttt() {
    let mut store = Store::new("");
    store.connect().await;
    let key = "root_app_key";
    let usr = store.find_user(key).await.unwrap();
    // dbg!(push(con, key, "7"));
    // dbg!(lrange(con, key, 0, 8));
    // dbg!(pop(con, key));
    // dbg!(range_push(con, key, "11".to_owned().as_bytes().to_vec(), 8));
    // dbg!(set(con, &usr, ClipType::Text, "content".as_bytes()));
    let con = &mut redis_store::connect();
    dbg!(redis_store::get(con, &usr));
}
