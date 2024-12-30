use serde::{Deserialize, Serialize};
use tokio::{fs, io};

#[derive(Serialize, Deserialize)]
pub struct Queue {
    pub name: String,
    pub size: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub queues: Vec<Queue>,
}

#[derive(Debug)]
pub enum ReadConfigError {
    IoError(io::Error),
    ParseError(serde_yaml::Error),
}

pub async fn read_config() -> Result<Config, ReadConfigError> {
    let file = fs::read_to_string("./river.config.yaml").await;
    match file {
        Ok(file) => match serde_yaml::from_str(&file) {
            Ok(config) => Ok(config),
            Err(error) => Err(ReadConfigError::ParseError(error)),
        },
        Err(error) => Err(ReadConfigError::IoError(error)),
    }
}
