mod logger;

use std::net::SocketAddr;

use anyhow::anyhow;
use tracing::info;

#[tokio::main]
async fn main() {
    bootstrap().await.unwrap_or_else(|err| {
        panic!("Bootstrap error: {}", err);
    });

    let addr = format!(
        "{}:{}",
        dsa_env::get_env().server_host,
        dsa_env::get_env().server_port
    );
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("TcpListener could not bind.");

    info!("DS Arena service has been started at {}. Enjoy it.", &addr);

    axum::serve(
        listener,
        dsa_web::get_app()
            .to_owned()
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown())
    .await
    .expect("Failed to start axum server.");
}

async fn bootstrap() -> Result<(), anyhow::Error> {
    logger::init().await;

    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| anyhow!("Failed to install `ring` as default crypto provider."))?;

    dsa_cache::init().await;

    dsa_checker::init().await?;
    dsa_web::init().await?;

    Ok(())
}

async fn shutdown() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down...");
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down...");
        }
    }

    std::process::exit(0);
}
