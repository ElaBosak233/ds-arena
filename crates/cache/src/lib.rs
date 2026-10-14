use std::any::Any;

use chrono::Utc;
use dashmap::DashMap;
use once_cell::sync::Lazy;

mod worker;

type CacheEntry = (Box<dyn Any + Send + Sync>, Option<i64>);

pub(crate) static CACHES: Lazy<DashMap<String, CacheEntry>> = Lazy::new(DashMap::new);

pub async fn init() {
    worker::cleaner().await;
}

pub async fn set<T>(key: String, value: T)
where
    T: Any + Send + Sync,
{
    CACHES.insert(key, (Box::new(value), None));
}

pub async fn set_ex<T>(key: String, value: T, ttl: i64)
where
    T: Any + Send + Sync,
{
    CACHES.insert(key, (Box::new(value), Some(Utc::now().timestamp() + ttl)));
}

pub async fn get<T>(key: &str) -> Option<T>
where
    T: Any + Send + Sync + Clone,
{
    if let Some(item) = CACHES.get(key) {
        let value = item.value();
        let (value, expires_at) = value;
        if expires_at.is_some_and(|expires_at| Utc::now().timestamp() >= expires_at) {
            return None;
        }
        value.downcast_ref::<T>().cloned()
    } else {
        None
    }
}

pub async fn get_del<T>(key: &str) -> Option<T>
where
    T: Any + Send + Sync + Clone,
{
    let (_, (value, expires_at)) = CACHES.remove(key)?;
    if expires_at.is_some_and(|expires_at| Utc::now().timestamp() >= expires_at) {
        return None;
    }
    value.downcast::<T>().ok().map(|value| *value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn stores_and_consumes_values() {
        set("test".to_string(), "test".to_string()).await;
        assert_eq!(get::<String>("test").await.unwrap(), "test".to_string());

        set_ex("test_ex".to_string(), "test_ex".to_string(), 5).await;
        assert_eq!(
            get::<String>("test_ex").await.unwrap(),
            "test_ex".to_string()
        );
        assert_eq!(get_del::<String>("test").await.as_deref(), Some("test"));
        assert_eq!(get_del::<String>("test").await, None);
    }

    #[tokio::test]
    async fn expired_values_are_unavailable_without_the_cleaner() {
        set_ex("expired_get".into(), "value".to_string(), 0).await;
        set_ex("expired_take".into(), "value".to_string(), -1).await;
        assert_eq!(get::<String>("expired_get").await, None);
        assert_eq!(get_del::<String>("expired_take").await, None);
    }

    #[tokio::test]
    async fn only_one_concurrent_reader_consumes_a_value() {
        set("consume_once".into(), "value".to_string()).await;
        let (first, second) = tokio::join!(
            get_del::<String>("consume_once"),
            get_del::<String>("consume_once")
        );
        assert_eq!(
            usize::from(first.is_some()) + usize::from(second.is_some()),
            1
        );
    }
}
