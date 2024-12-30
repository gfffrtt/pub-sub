use river::{config::read_config, message::Payload, queue::Queues};
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = read_config().await.expect("Failed to read config file");
    let queues = Arc::new(Queues::new(&config));
    let listeners = TcpListener::bind(config.host)
        .await
        .expect("Failed to start TCP server");
    let (tx, mut rx) = channel::<Payload>(256);
    let queues_clone = Arc::clone(&queues);
    let tx_arc = Arc::new(tx);

    tokio::spawn(async move {
        loop {
            let payload = rx.recv().await.expect("Failed to receive payload");

            let mut queues_guard = queues_clone
                .queues
                .write()
                .expect("Failed to lock queues for writing");

            let queue = queues_guard
                .get_mut(&payload.queue)
                .expect("Queue not found");

            let subscribers_guard = queue
                .subscribers
                .read()
                .expect("Failed to lock subscribers for reading");

            subscribers_guard.iter().for_each(|subscriber| {
                subscriber
                    .try_write(&payload.message)
                    .expect("Failed to write to subscriber");
            });
        }
    });

    loop {
        let (mut socket, _) = listeners
            .accept()
            .await
            .expect("Failed to accept connection");

        let queues_clone = Arc::clone(&queues);
        let sender = tx_arc.clone();

        tokio::spawn(async move {
            let buffer = &mut [0; 1024];

            let n = socket
                .read(buffer)
                .await
                .expect("Failed to read from socket");

            let configuration = String::from_utf8(buffer[..n].to_vec()).expect("Failed to parse");

            let (queue_name, kind) = configuration
                .split_once(":")
                .expect("Invalid configuration");

            let queues_write = queues_clone
                .queues
                .read()
                .expect("Failed to acquire write lock");

            let queue = queues_write.get(queue_name).expect("Queue not found");

            match kind {
                "sub" => {
                    let mut subscribers_lock = queue
                        .subscribers
                        .write()
                        .expect("Failed to acquire write lock");

                    subscribers_lock.push(socket);
                }
                "pub" => {
                    let queue_name_clone = queue_name.to_string();
                    tokio::spawn(async move {
                        loop {
                            let buffer = &mut [0; 4096];
                            let n = socket
                                .read(buffer)
                                .await
                                .expect("Failed to read from socket");
                            sender
                                .send(Payload::new(queue_name_clone.clone(), buffer[..n].to_vec()))
                                .await
                                .expect("Failed to send payload");
                        }
                    });
                }
                _ => println!("Invalid kind"),
            }
        });
    }
}
