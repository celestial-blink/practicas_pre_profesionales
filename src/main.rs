mod config;
mod general_types;
mod helpers;
mod maud;
mod middleware;
mod modules;
mod t_logs;
mod types;
mod macros;
mod data;

use crate::middleware::api_auth_middleware::api_auth_middleware;
use crate::modules::convocatorias::presentation::router as convocatoria_router;
use crate::modules::ofertas::presentation::router as oferta_router;
use crate::modules::organizaciones::presentation::router as organizacion_router;
use crate::{general_types::State, maud::pages::filters::page_filters};

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

    let _ = t_logs::init();

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
    let storage_dir = std::env::var("STORAGE_DIR").expect("STORAGE_DIR must be set");

    let _ = HttpServer::new(move || {
        App::new()
            .service(
                actix_files::Files::new("/public", &storage_dir)
                    // .show_files_listing()
                    .use_last_modified(true),
            )
            .wrap(TracingLogger::default())
            .service(maud::pages::home::index::home_index)
            .service(maud::pages::convocatorias_practicas::index::convocatorias_practicas)
            .service(maud::pages::oferta_practicas::index::oferta_practicas)
            .service(maud::pages::departamentos::index::departamentos_view)
            .service(maud::pages::organizaciones::index::organizaciones_view)
            .service(maud::pages::busqueda::index::busqueda_view)
            .service(page_filters)
            .app_data(web::Data::new(State { db: pool.clone() }))
            .app_data(TempFileConfig::default().directory(&temp_dir))
            .service(
                web::scope("/api/v1")
                    .wrap(from_fn(api_auth_middleware))
                    .route("/validate-auth", web::get().to(|| async { "Is valid" }))
                    .service(web::scope("/pre-ofertas").service(
                        modules::pre_ofertas::presentation::router::insert_many::insert_many
                    ))
                    .service(
                        web::scope("/organizaciones")
                            .service(organizacion_router::find_by_search::find_by_search)
                            .service(organizacion_router::create::create)
                            .service(organizacion_router::update::update)
                            .service(organizacion_router::find_by_id::find_by_id)
                            .service(organizacion_router::find_by_ruc::find_by_ruc)
                    )
                    .service(
                        web::scope("/ofertas")
                            .service(oferta_router::find_by_search::find_by_search)
                            .service(oferta_router::create::create)
                            .service(oferta_router::update::update)
                            .service(oferta_router::find_by_id::find_by_id)
                            .service(oferta_router::find_by_id_with_niveles::find_by_id_with_niveles)
                            .service(oferta_router::get_all_by_id_convocatoria::get_all_by_id_convocatoria) // resumen
                    )
                    .service(
                        web::scope("/convocatorias")
                            .service(convocatoria_router::find_by_search::find_by_search)
                            .service(convocatoria_router::find_by_search_for_list::find_by_search_for_list)
                            .service(convocatoria_router::find_by_id_for_list::find_by_id_for_list)
                            .service(convocatoria_router::create::create)
                            .service(convocatoria_router::update::update)
                            .service(convocatoria_router::find_by_id::find_by_id)
                    )
            )
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await;

    Ok(())
}
