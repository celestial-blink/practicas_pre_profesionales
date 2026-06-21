use time::{UtcOffset, format_description::well_known::Rfc3339};
use tracing_appender::rolling;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::config::IS_DEV;

pub fn init() -> std::io::Result<()> {
    let lima_offset = UtcOffset::from_hms(-5, 0, 0).unwrap();

    let registry = tracing_subscriber::registry().with(EnvFilter::from_default_env());

    if IS_DEV {
        registry
            .with(fmt::layer().with_timer(OffsetTime::new(lima_offset, Rfc3339)))
            .init();
    } else {
        let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());

        let file_appender = rolling::daily(&log_dir, "app.log");
        let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

        registry
            .with(
                fmt::layer()
                    .with_timer(OffsetTime::new(lima_offset, Rfc3339))
                    .with_writer(file_writer),
            )
            .init();
    }

    Ok(())
}
