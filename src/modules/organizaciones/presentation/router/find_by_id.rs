use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::organizaciones::{
        application::find_by_id::FindById, infrastructure::mariadb_repository::MariadbRepository,
    },
};

#[get("/{id}")]
pub async fn find_by_id(state: web::Data<State>, params: web::Path<i32>) -> impl Responder {
    let id = params.into_inner();
    let infrastructure = MariadbRepository::new(state.db.clone());
    let application = FindById::new(infrastructure);
    let result = application.execute(id).await;
    match result {
        Some(organizacion) => HttpResponse::Ok().json(organizacion),
        None => HttpResponse::NotFound().body("Organizacion no encontrada"),
    }
}
