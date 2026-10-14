pub async fn cleaner() {
    tokio::spawn(async {
        let interval = std::time::Duration::from_secs(1);
        loop {
            let now = chrono::Utc::now().timestamp();
            super::CACHES
                .retain(|_, (_, expires_at)| expires_at.is_none_or(|expires_at| now < expires_at));

            tokio::time::sleep(interval).await;
        }
    });
}
