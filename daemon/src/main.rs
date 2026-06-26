use chrono::Local;
use dirs::config_dir;
use protocol::SOCKET_PATH;
use protocol::{Request, Response};
use tokio::{
    fs::{OpenOptions, create_dir_all, remove_file, try_exists},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{UnixListener, UnixStream},
    time::Duration,
};

#[tokio::main]
async fn main() {
    tokio::join!(logging(), socket_listener());
}

async fn socket_listener() {
    if try_exists(SOCKET_PATH).await.unwrap_or(false) {
        remove_file(SOCKET_PATH)
            .await
            .expect("error occured while removing file, restart the process");
    }

    let socket = UnixListener::bind(SOCKET_PATH).unwrap();

    loop {
        match socket.accept().await {
            Ok((stream, _addr)) => {
                tokio::spawn(async move {
                    handle_client(stream).await;
                });
            }
            Err(e) => println!("some error occured {:?}", e),
        }
    }
}

// background worker for concurrent streams
// extra stream ahile aitu e handle koribo
async fn handle_client(mut stream: UnixStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut line = String::new();
    let _bytes_read = match reader.read_line(&mut line).await {
        Ok(0) => return,
        Ok(size) => size,
        Err(e) => {
            println!("Error reading from stream: {:?}", e);
            return;
        }
    };
    // mur mur tu
    // let actual_data = std::str::from_utf8(&buffer[0..bytes_read]).expect("error in conversion");

    let request: Request = match serde_json::from_str(&line) {
        Ok(req) => req,
        Err(e) => {
            // error message here, first stdout, then json response sent
            println!("Failed to parse JSON from client {:?}", e);
            let err_res = Response {
                ok: false,
                message: None,
                error: Some("Invalid JSON format".to_string()),
            };

            let mut err_json = serde_json::to_string(&err_res).unwrap();
            err_json.push('\n');
            stream.write_all(err_json.as_bytes()).await.unwrap();
            return;
        }
    };

    let response = match request {
        Request::Ping => Response {
            ok: true,
            message: Some("pong".to_string()),
            error: None,
        },
        Request::Status => Response {
            ok: true,
            message: Some("Daemon is working properly".to_string()),
            error: None,
        },
        Request::Stop => Response {
            ok: true,
            message: Some("Shutting down daemon".to_string()),
            error: None,
        },
    };

    let mut json_reply = serde_json::to_string(&response).unwrap();
    json_reply.push('\n');
    stream
        .write_all(json_reply.as_bytes())
        .await
        .expect("Failed to send response")
}

async fn logging() {
    println!("logging ");

    let base = config_dir()
        .expect("config dir not found")
        .join("secretwarden");

    let log_dir = base.join("logs");
    let bin_dir = base.join("bin");
    let log_file = log_dir.join("daemon.log");
    let stdout_file = log_dir.join("stdout.log");
    let stderr_file = log_dir.join("stderr.log");

    create_dir_all(&base)
        .await
        .expect("couldn't create directory");
    create_dir_all(&bin_dir)
        .await
        .expect("couldn't create directory");
    create_dir_all(&log_dir)
        .await
        .expect("couldn't create directory");

    let mut file1 = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&log_file)
        .await
        .expect("file not found or some error happened");

    let mut _file2 = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&stdout_file)
        .await
        .expect("file not found or some error happened");

    let mut _file3 = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&stderr_file)
        .await
        .expect("file not found or some error happened");

    let mut i = 0;
    loop {
        let now = Local::now();
        let message = format!(
            "
        this current moment: {} and incremented nos: {}\n
        ",
            now,
            {
                let tmp = i;
                i += 1;
                tmp
            }
        );

        file1
            .write_all(message.as_bytes())
            .await
            .expect("couldn't write");

        // changed from
        // thread::sleep(ten_secs); to the following
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
