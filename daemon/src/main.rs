use std::{
    fs::{self, OpenOptions},
    thread::{self},
    time,
};

use chrono::Local;
use dirs::config_dir;
use std::io::Write;

fn main() {
    let base = config_dir()
        .expect("config dir not found")
        .join("secretwarden");

    let log_dir = base.join("logs");
    let log_file = log_dir.join("daemon.log");

    fs::create_dir_all(&base).expect("couldn't create directory");
    fs::create_dir_all(&log_dir).expect("couldn't create directory");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&log_file)
        .expect("file not found or some error happened");

    let mut i = 0;
    loop {
        let now = Local::now();

        writeln!(
            file,
            "I love gargu at this current time: {} and also {} many times",
            now,
            {
                let tmp = i;
                i += 1;
                tmp
            }
        )
        .expect("couldn't write");

        let ten_secs = time::Duration::from_secs(2);

        thread::sleep(ten_secs);
    }
}
