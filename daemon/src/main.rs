use chrono::Local;
use dirs::config_dir;
use tokio::{
    fs::{OpenOptions, create_dir_all, remove_file, try_exists},
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixListener,
    time::Duration,
};

#[tokio::main]
async fn main() {
    tokio::join!(logging(), socket_listener(),);
}

const SOCKET_PATH: &str = "/tmp/secretwarden.sock";

async fn socket_listener() {
    if try_exists(SOCKET_PATH).await.unwrap_or(false) {
        remove_file(SOCKET_PATH)
            .await
            .expect("error occured while removing file, restart the process");
    }

    let socket = UnixListener::bind(SOCKET_PATH).unwrap();

    /* accept returns this
    Result<(UnixStream, SocketAddr), Error>

      Success
      ↓
      (Stream, Address)

    */

    loop {
        match socket.accept().await {
            Ok((mut stream, _addr)) => {
                let mut buffer = vec![0; 1024]; // creates a storage space with buffer name
                let bytes_read = stream.read(&mut buffer).await; // fills that storage and returns
                // the number of bytes of data inserted
                let buffer_data = &buffer[0..bytes_read] match {
                   Ok(file) 
                }; // takes out the bytes of data inserted
                // in the buffer
                let actual_data = std::str::from_utf8(buffer_data).expect("error in conversion"); // the
                // bytes is converted to string utf8
                if actual_data == "ping" {
                    println!("pong");
                }
            } // _ has been used to show
            // that neither variables are being intentionally used although created. inshort ignore
            Err(e) => println!("some error occured {:?}", e),
        }
    }
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
