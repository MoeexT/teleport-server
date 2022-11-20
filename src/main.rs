
// use log::{info, error};
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncWriteExt, AsyncReadExt}};

pub mod db;

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }

}

async fn process(mut socket: TcpStream) {
    let mut buf = Vec::new();
    let res = socket.read_to_end(&mut buf).await;
    match res {
        Ok(len) => {
            println!("{}", len);
            // println!("{}", String::from_utf8_lossy(&buf[..len]));
            // info!("{}", String::from_utf8_lossy(&buf[..]));
            socket.write("HTTP/1.1 200 OK \r\n\r\n Hello World!".as_bytes()).await.unwrap();
        },
        Err(err) => println!("{:?}", err)
    }
}

