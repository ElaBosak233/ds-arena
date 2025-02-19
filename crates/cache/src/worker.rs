pub async fn cleaner() {
    tokio::spawn(async {
        let interval = std::time::Duration::from_secs(1);
        loop {
            let now = chrono::Utc::now().timestamp();
            let keys_to_remove: Vec<String> = super::CACHES
                .iter()
                .filter_map(|item| {
                    let expire = item.value().1?;
                    if now >= expire {
                        Some(item.key().clone())
                    } else {
                        None
                    }
                })
                .collect();

            for key in keys_to_remove {
                super::CACHES.remove(&key);
            }

            tokio::time::sleep(interval).await;
        }
    });
}
