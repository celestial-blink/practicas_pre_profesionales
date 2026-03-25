mod config;
mod general_types;
mod middleware;
mod modules;
mod maud;
mod t_logs;
mod helpers;

use crate::maud::pages::home::home_index;
use crate::modules::organizaciones::presentation::router::find_by_search::find_by_search;
use crate::{general_types::State};
use crate::middleware::api_auth_middleware::api_auth_middleware;

use actix_web::{App, HttpServer, middleware::from_fn, web};
use dotenvy::dotenv;
use sqlx::MySqlPool;

use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = std::env::var("PORT").expect("PORT must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let _ = t_logs::init().await;

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
            .service(actix_files::Files::new("/public", "./public").show_files_listing().use_last_modified(true))
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(State { db: pool.clone() }))
            .service(
                web::scope("/api/v1")
                    .wrap(from_fn(api_auth_middleware))
                    .route("/test", web::get().to(|| async { "Test" }))
                    .service(web::scope("/pre-ofertas").service(
                        modules::pre_ofertas::presentation::router::insert_many::insert_many,
                    ))
                    .service(web::scope("/organizaciones").service(
                        find_by_search,
                    ))
            )
            .service(home_index)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await;

    Ok(())
}
