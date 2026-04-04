use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::ofertas::{
        application::find_by_id::FindById,
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[get("/{id}")]
pub async fn find_by_id(state: web::Data<State>, params: web::Path<i32>) -> impl Responder {
    let id = params.into_inner();
    let infrastructure = MariaDbRepository::new(state.db.clone());
    let application = FindById::new(infrastructure);
    let result = application.execute(id).await;

    match result {
        Some(oferta) => HttpResponse::Ok().json(oferta),
        None => HttpResponse::NotFound().body("Oferta no encontrada"),
    }
}
