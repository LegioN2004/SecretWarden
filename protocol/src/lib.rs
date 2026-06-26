use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Ping,
    Status,
    Stop,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}

pub const SOCKET_PATH: &str = "/tmp/secretwarden.sock";
