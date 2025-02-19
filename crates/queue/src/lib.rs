use std::{any::Any, sync::Arc};

use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::mpsc;

static QUEUES: Lazy<Arc<DashMap<String, mpsc::Sender<Box<dyn Any + Send + Sync>>>>> =
    Lazy::new(|| Arc::new(DashMap::new()));

pub async fn publish<T>(channel: &str, msg: T)
where
    T: Any + Send + Sync, {
    if let Some(tx) = QUEUES.get(channel) {
        if tx.send(Box::new(msg)).await.is_err() {
            println!("Failed to send message to channel '{}'", channel);
        }
    } else {
        println!("Channel '{}' does not exist", channel);
    }
}

pub async fn subscribe(channel: &str) -> mpsc::Receiver<Box<dyn Any + Send + Sync>> {
    let (tx, rx) = mpsc::channel(100); // 可以调整缓冲区大小
    QUEUES.insert(channel.to_string(), tx);

    rx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let mut rx = subscribe("test").await;
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Some(msg) = msg.downcast_ref::<String>() {
                    println!("Received string message: {}", msg);
                } else if let Some(num) = msg.downcast_ref::<i32>() {
                    println!("Received number message: {}", num);
                } else {
                    println!("Received unknown message type");
                }
            }
        });

        publish("test", "Hello, world!".to_string()).await;
        publish("test", "Hello, Ela!".to_string()).await;
        publish("test", 20101010).await;
    }
}
