use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::pre_ofertas::{
        application::find_by_id::FindById,
        infrastructure::persistence::mariadb_repository::MariadbRepository,
    },
};

#[get("/{id}")]
pub async fn find_by_id(state: web::Data<RwLock<State>>, params: web::Path<i32>) -> impl Responder {
    let id = params.into_inner();
    let infrastructure = MariadbRepository::new(state.read().unwrap().db.clone());
    let application = FindById::new(infrastructure);
    let result = application.execute(id).await;

    match result {
        Some(pre_oferta) => HttpResponse::Ok().json(pre_oferta),
        None => HttpResponse::NotFound().body("Pre-oferta no encontrada"),
    }
}
