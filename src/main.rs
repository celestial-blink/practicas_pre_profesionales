mod config;
mod general_types;
mod middleware;
mod modules;
use crate::general_types::State;
use crate::middleware::api_auth_middleware::api_auth_middleware;

use actix_web::{App, HttpServer, middleware::from_fn, web};
use dotenvy::dotenv;

use sqlx::MySqlPool;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").expect("PORT must be set");

    let pool = MySqlPool::connect(&database_url).await.unwrap();

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State { db: pool.clone() }))
            .service(
                web::scope("/api/v1")
                    .wrap(from_fn(api_auth_middleware))
                    .route("/test", web::get().to(|| async { "Test" }))
                    .service(web::scope("/pre-ofertas").service(
                        modules::pre_ofertas::presentation::router::insert_many::insert_many,
                    )),
            )
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await;

    println!("Server running on port {}", port);

    Ok(())
}
