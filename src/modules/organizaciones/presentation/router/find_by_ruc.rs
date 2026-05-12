use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::organizaciones::{
        application::find_by_ruc::FindByRuc, infrastructure::mariadb_repository::MariadbRepository,
    },
};

#[get("/ruc/{ruc}")]
pub async fn find_by_ruc(
    state: web::Data<RwLock<State>>,
    params: web::Path<String>,
) -> impl Responder {
    let ruc = params.into_inner();
    let infrastructure = MariadbRepository::new(state.read().unwrap().db.clone());
    let application = FindByRuc::new(infrastructure);
    let result = application.execute(ruc).await;
    match result {
        Some(organizacion) => HttpResponse::Ok().json(organizacion),
        None => HttpResponse::NotFound().body("Organizacion no encontrada"),
    }
}
