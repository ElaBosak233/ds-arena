use std::{
    backtrace::{Backtrace, BacktraceStatus},
    panic,
    path::Path,
    thread::sleep,
};

use once_cell::sync::OnceCell;
use tracing::{Level, error, info, info_span, warn};
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    EnvFilter, Layer, Registry, layer::SubscriberExt, util::SubscriberInitExt,
};

static CONSOLE_GUARD: OnceCell<WorkerGuard> = OnceCell::new();

pub async fn init() {
    let filter = EnvFilter::new(&dsa_env::get_env().logger_level);

    let (non_blocking_console, console_guard) = non_blocking(std::io::stdout());

    let console_layer = tracing_subscriber::fmt::Layer::new()
        .with_writer(non_blocking_console)
        .with_ansi(true)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false);

    let subscriber = Registry::default()
        .with(ErrorLayer::default())
        .with(filter)
        .with(console_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    panic::set_hook(Box::new(|panic_hook_info| {
        let payload = panic_hook_info.payload();
        let payload = if let Some(s) = payload.downcast_ref::<&str>() {
            &**s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.as_str()
        } else {
            ""
        };

        let location = panic_hook_info.location().map(|l| l.to_string());

        let (backtrace, note) = {
            let backtrace = Backtrace::capture();
            let note = (backtrace.status() == BacktraceStatus::Disabled)
                .then_some("run with RUST_BACKTRACE=1 environment variable to display a backtrace");
            (Some(backtrace), note)
        };

        error!(
            target: "panic_hook",
            location = location,
            backtrace = backtrace.map(display),
            note = note,
            "{}", payload
        );

        warn!(
            target: "panic_hook",
            "Oops, it seems that a panic has occurred, this thread will shut down in 5 seconds."
        );

        sleep(std::time::Duration::from_secs(5));
    }));

    info!("Logger initialized successfully.");

    CONSOLE_GUARD.set(console_guard).unwrap();
}
