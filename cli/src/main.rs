use std::env::args;
use tokio::{
    fs::try_exists,
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

const SOCKET_PATH: &str = "/tmp/secretwarden.sock";

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Usage: sw command args");
        return;
    }

    let command = &args[1];

    send_req(command).await;
}

async fn send_req(command: &str) {
    // sending ping
    if try_exists(&SOCKET_PATH).await.unwrap_or(false) {
        let mut connection = UnixStream::connect(SOCKET_PATH)
            .await
            .expect("failed to connect: "); // expect appends the error at the end of the string so
        // no need of adding another {:?}
        connection
            .write_all(command.as_bytes())
            .await
            .expect("some error occured while sending");

        // connection.flush().await.expect("error while flushing");
        println!("connected");

        let mut buffer = vec![0; 1024];
        let bytes_read = match connection.read(&mut buffer).await {
            Ok(0) => return,
            Ok(size) => size,
            Err(e) => {
                println!("Error reading from stream: {:?}", e);
                return;
            }
        };
        let response = std::str::from_utf8(&buffer[0..bytes_read]).expect("error in conversion");
        println!("{}", response);
    } else {
        println!("daemon is not running(socket not found)");
    }
}
