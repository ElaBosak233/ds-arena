use std::{any::Any, sync::Arc};

use chrono::Utc;
use dashmap::DashMap;
use once_cell::sync::Lazy;

mod worker;

pub(crate) static CACHES: Lazy<Arc<DashMap<String, (Box<dyn Any + Send + Sync>, Option<i64>)>>> =
    Lazy::new(|| Arc::new(DashMap::new()));

pub async fn init() {
    worker::cleaner().await;
}

pub async fn set<T>(key: String, value: T)
where
    T: Any + Send + Sync, {
    CACHES.insert(key, (Box::new(value), None));
}

pub async fn set_ex<T>(key: String, value: T, ttl: i64)
where
    T: Any + Send + Sync, {
    CACHES.insert(key, (Box::new(value), Some(Utc::now().timestamp() + ttl)));
}

pub async fn get<T>(key: &str) -> Option<T>
where
    T: Any + Send + Sync + Clone, {
    if let Some(item) = CACHES.get(key) {
        let value = item.value();
        let (value, _) = value;
        value.downcast_ref::<T>().map(|v| v.clone())
    } else {
        None
    }
}

pub async fn get_del<T>(key: &str) -> Option<T>
where
    T: Any + Send + Sync + Clone, {
    let value = get::<T>(key).await;
    CACHES.remove(key);

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        init().await;
        set("test".to_string(), "test".to_string()).await;
        assert_eq!(get::<String>("test").await.unwrap(), "test".to_string());

        set_ex("test_ex".to_string(), "test_ex".to_string(), 5).await;
        assert_eq!(
            get::<String>("test_ex").await.unwrap(),
            "test_ex".to_string()
        );
        tokio::time::sleep(std::time::Duration::from_secs(7)).await;
        assert_eq!(get::<String>("test_ex").await, None);
    }
}
