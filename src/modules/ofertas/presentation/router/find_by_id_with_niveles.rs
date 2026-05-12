use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::ofertas::{
        application::find_by_id_with_niveles::FindByIdWithNiveles,
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[get("/{id}/niveles")]
pub async fn find_by_id_with_niveles(
    state: web::Data<RwLock<State>>,
    params: web::Path<i32>,
) -> impl Responder {
    let id = params.into_inner();
    let infrastructure = MariaDbRepository::new(state.read().unwrap().db.clone());
    let application = FindByIdWithNiveles::new(infrastructure);
    let result = application.execute(id).await;

    match result {
        Some(oferta) => HttpResponse::Ok().json(oferta),
        None => HttpResponse::NotFound().body("Oferta no encontrada"),
    }
}
