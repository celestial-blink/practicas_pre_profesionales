use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() -> std::io::Result<()> {
    let registry = tracing_subscriber::registry().with(EnvFilter::from_default_env());

    // if IS_DEV {
    registry.with(fmt::layer().without_time()).init();
    // } else {
    //     let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());

    //     let file_appender = rolling::daily(&log_dir, "app.log");
    //     let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    //     registry
    //         .with(
    //             fmt::layer()
    //                 .with_timer(OffsetTime::new(lima_offset, Rfc3339))
    //                 .with_writer(file_writer),
    //         )
    //         .init();
    // }

    Ok(())
}
