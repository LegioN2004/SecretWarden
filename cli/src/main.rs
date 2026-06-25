use tokio::{fs::try_exists, net::UnixStream};

// fn main() {
//     println!("Hello, world!");
// }

const SOCKET_PATH: &str = "/tmp/secretwarden.sock";

#[tokio::main]
async fn main() {
    send_req().await;
}

async fn send_req() {
    if try_exists(&SOCKET_PATH).await.unwrap_or(false) {
        let _ = UnixStream::connect(SOCKET_PATH)
            .await
            .expect("failed to connect: "); // expect appends the error at the end of the string so
        // no need of adding another {:?}
        println!("connected");
    } else {
        println!("daemon is not running(socket not found)");
    }
}
