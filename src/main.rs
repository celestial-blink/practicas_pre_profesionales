mod config;
mod general_types;
mod middleware;
mod modules;
use std::fs::OpenOptions;

use crate::general_types::State;
use crate::middleware::api_auth_middleware::api_auth_middleware;

use actix_web::{App, HttpServer, middleware::from_fn, web};
use dotenvy::dotenv;
use sqlx::MySqlPool;

use time::{
    OffsetDateTime, UtcOffset,
    format_description::{self, well_known::Rfc3339},
};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

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

    let port = std::env::var("PORT").expect("PORT must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    tracing::info!("🚀 Servidor iniciando en http://127.0.0.1:{}", port);

    let pool = match MySqlPool::connect(&database_url).await {
        Ok(pool) => {
            tracing::info!("Conexión a base de datos exitosa");
            pool
        }
        Err(e) => {
            tracing::error!("Error conectando a la base de datos: {}", e);
            return Ok(());
        }
    };

    let _ = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(State { db: pool.clone() }))
            .service(
                web::scope("/api/v1")
                    .wrap(from_fn(api_auth_middleware))
                    .route("/test", web::get().to(|| async { "Test" }))
                    .service(web::scope("/pre-ofertas").service(
                        modules::pre_ofertas::presentation::router::insert_many::insert_many,
                    )),
            )
            .route("/test", web::get().to(|| async { "Test" }))
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await;

    Ok(())
}
