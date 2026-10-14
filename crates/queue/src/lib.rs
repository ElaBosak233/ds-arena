use std::any::Any;

use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::mpsc;

type Message = Box<dyn Any + Send + Sync>;

static QUEUES: Lazy<DashMap<String, mpsc::Sender<Message>>> = Lazy::new(DashMap::new);

pub async fn publish<T>(channel: &str, msg: T)
where
    T: Any + Send + Sync,
{
    let tx = QUEUES.get(channel).map(|tx| tx.value().clone());
    if let Some(tx) = tx {
        if tx.send(Box::new(msg)).await.is_err() {
            println!("Failed to send message to channel '{}'", channel);
        }
    } else {
        println!("Channel '{}' does not exist", channel);
    }
}

pub async fn subscribe(channel: &str) -> mpsc::Receiver<Message> {
    let (tx, rx) = mpsc::channel(100); // 可以调整缓冲区大小
    QUEUES.insert(channel.to_string(), tx);

    rx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn delivers_values_to_the_subscriber() {
        let mut rx = subscribe("test").await;
        publish("test", "Hello, world!".to_string()).await;
        let message = rx.recv().await.unwrap();
        assert_eq!(message.downcast_ref::<String>().unwrap(), "Hello, world!");
    }
}
