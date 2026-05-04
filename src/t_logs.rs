use std::fs::OpenOptions;

use time::{
    OffsetDateTime, UtcOffset,
    format_description::{self, well_known::Rfc3339},
};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn init() -> std::io::Result<()> {
    let lima_offset = UtcOffset::from_hms(-5, 0, 0).unwrap();

    let registry = tracing_subscriber::registry().with(EnvFilter::from_default_env());

    let is_dev = std::env::var("IS_DEV").unwrap_or_else(|_| "false".to_string());

    if is_dev == "true" {
        registry
            .with(fmt::layer().with_timer(OffsetTime::new(lima_offset, Rfc3339)))
            .init();
    } else {
        let now = OffsetDateTime::now_local().unwrap();
        let format = format_description::parse("[year]-[month]-[day]").unwrap();
        let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());

        let filename = format!("{}/app_{}.log", log_dir, now.format(&format).unwrap());

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)?;

        registry
            .with(
                fmt::layer()
                    .with_timer(OffsetTime::new(lima_offset, Rfc3339))
                    .with_writer(log_file),
            )
            .init();
    }

    // .with(EnvFilter::new("info").add_directive("sqlx=debug".parse().unwrap()))

    // registry.init(); // esto ya inicializa el bridge con log internamente
    Ok(())
}
