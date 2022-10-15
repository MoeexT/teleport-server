use redis::ConnectionLike;
// use redis::types::Value;


#[test]
fn connect_server() {
    let client = redis::Client::open("redis://localhost/").unwrap();
    let mut con = client.get_connection().unwrap();
    println!("Is open: {}", con.is_open());
    // con.keys("*");
    // redis::cmd("keys").arg("*").query(&mut con).unwrap();
    // let _ : () = redis::cmd("SET").arg("a_key").arg("arg").query(&mut con).unwrap();
    // let res: String = redis::cmd("GET").arg("a_key").query(&mut con).unwrap();
    // let res: usize = redis::cmd("exists").arg("a_key").query(&mut con).unwrap();
    let res: Vec<String> = redis::cmd("keys").arg("*").query(&mut con).unwrap();
    for s in res {
        println!("Result: {}", s);
    }
    
}

