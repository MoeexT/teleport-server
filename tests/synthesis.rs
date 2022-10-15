use teleport_server::db::{mysql_store, redis_store};

#[tokio::test]
#[ignore = "Not insert value to redis"]
async fn ttt() {
    let db = &mysql_store::connect().await;
    let con = &mut redis_store::connect();
    let key = "root_app_key";
    let usr = mysql_store::find_user(db, key).await.unwrap();
    // dbg!(push(con, key, "7"));
    // dbg!(lrange(con, key, 0, 8));
    // dbg!(pop(con, key));
    // dbg!(range_push(con, key, "11".to_owned().as_bytes().to_vec(), 8));
    // dbg!(set(con, &usr, ClipType::Text, "content".as_bytes()));
    dbg!(redis_store::get(con, &usr));
}
