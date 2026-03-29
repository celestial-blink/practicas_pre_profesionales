mod config;
mod general_types;
mod helpers;
mod maud;
mod middleware;
mod modules;
mod t_logs;

use crate::{general_types::State, modules::organizaciones::presentation::router::update::update};
use crate::maud::pages::home::home_index;
use crate::middleware::api_auth_middleware::api_auth_middleware;
use crate::modules::organizaciones::presentation::router::create::create;
use crate::modules::organizaciones::presentation::router::find_by_search::find_by_search;

use actix_multipart::form::tempfile::TempFileConfig;
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

    let temp_dir = std::env::var("TEMP_DIR").expect("TEMP_DIR must be set");

//     println!("exists: {}", std::path::Path::new(&temp_dir).exists());
//     println!("is_dir: {}", std::path::Path::new(&temp_dir).is_dir());
// println!("metadata: {:?}", std::fs::metadata(&temp_dir));

// // Agrega esto también para ver el error exacto al escribir
// match std::fs::File::create(format!("{}/test.txt", &temp_dir)) {
//     Ok(_) => println!("Escritura OK"),
//     Err(e) => println!("Error escritura: {}", e),
// }

    let _ = HttpServer::new(move || {
        App::new()
            .service(
                actix_files::Files::new("/public", "./public")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(State { db: pool.clone() }))
            .app_data(TempFileConfig::default().directory(&temp_dir))
            .service(
                web::scope("/api/v1")
                    .wrap(from_fn(api_auth_middleware))
                    .route("/validate-auth", web::get().to(|| async { "Is valid" }))
                    .service(web::scope("/pre-ofertas").service(
                        modules::pre_ofertas::presentation::router::insert_many::insert_many,
                    ))
                    .service(
                        web::scope("/organizaciones")
                            .service(find_by_search)
                            .service(create)
                            .service(update),
                    ),
            )
            .service(home_index)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await;

    Ok(())
}
