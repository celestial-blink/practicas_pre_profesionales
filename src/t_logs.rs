use std::fs::OpenOptions;

use time::{OffsetDateTime, UtcOffset, format_description::{self, well_known::Rfc3339}};
use tracing_subscriber::{EnvFilter, fmt::{self, time::OffsetTime}, layer::SubscriberExt, util::SubscriberInitExt};



pub async fn init() -> std::io::Result<()> {
    let now = OffsetDateTime::now_local().unwrap();
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    let filename = format!("logs/app_{}.log", now.format(&format).unwrap());
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)?;

    let lima_offset = UtcOffset::from_hms(-5, 0, 0).unwrap();

    tracing_subscriber::registry()
        .with(EnvFilter::new("info").add_directive("sqlx=debug".parse().unwrap()))
        .with(
            fmt::layer()
                .with_timer(OffsetTime::new(lima_offset, Rfc3339))
                .with_writer(log_file),
        )
        .with(fmt::layer().with_timer(OffsetTime::new(lima_offset, Rfc3339)))
        .init(); // esto ya inicializa el bridge con log internamente
    Ok(())
}
