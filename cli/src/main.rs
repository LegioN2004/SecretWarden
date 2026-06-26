use protocol::SOCKET_PATH;
use protocol::{Request, Response};
use serde_json::to_string;
use std::env::args;
use tokio::{
    fs::try_exists,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
};

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

        let req_enum = match command {
            "ping" => Request::Ping,
            "status" => Request::Status,
            "stop" => Request::Stop,
            _ => {
                println!("Error: Unknown command '{}'", command);
                return;
            }
        };

        let mut json_msg = to_string(&req_enum).expect("Serialization failed");
        json_msg.push('\n'); // appends the given char to the end of the var

        connection
            .write_all(json_msg.as_bytes())
            .await
            .expect("some error occured while sending");

        let mut reader = BufReader::new(connection);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .await
            .expect("error reading response");

        let response: Response = serde_json::from_str(&line).expect("Invalid JSON received");

        if response.ok {
            println!("Success: {}", response.message.unwrap_or_default());
        } else {
            println!("Error: {}", response.error.unwrap_or_default());
        }

        // let mut buffer = vec![0; 1024];
        // let bytes_read = match connection.read(&mut buffer).await {
        //     Ok(0) => return,
        //     Ok(size) => size,
        //     Err(e) => {
        //         println!("Error reading from stream: {:?}", e);
        //         return;
        //     }
        // };
        // let response = std::str::from_utf8(&buffer[0..bytes_read]).expect("error in conversion");
    } else {
        println!("daemon is not running(socket not found)");
    }
}
