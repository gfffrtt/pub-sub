use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tokio::net::TcpStream;

use crate::config::Config;

pub struct Queue {
    pub size: u16,
    pub subscribers: Arc<RwLock<Vec<TcpStream>>>,
}

impl Queue {
    fn new(size: u16) -> Self {
        Self {
            size,
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

pub struct Queues {
    pub queues: Arc<RwLock<HashMap<String, Queue>>>,
}

impl Queues {
    pub fn new(config: &Config) -> Self {
        let mut queues = HashMap::new();
        config.queues.iter().for_each(|queue| {
            queues.insert(queue.name.to_string(), Queue::new(queue.size));
        });
        Self {
            queues: Arc::new(RwLock::new(queues)),
        }
    }
}
