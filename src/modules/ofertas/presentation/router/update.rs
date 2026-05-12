use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, put, web};

use crate::{
    general_types::State,
    modules::oferta_niveles::infrastructure::persitense::mariadb_repository::MariaDbRepository as MariaDbRepositoryNiveles,
    modules::ofertas::{
        application::update::Update, domain::dtos::update_dto::UpdateOfertaDto,
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[put("/{id}")]
pub async fn update(
    state: web::Data<RwLock<State>>,
    path: web::Path<i32>,
    params: web::Json<UpdateOfertaDto>,
) -> impl Responder {
    let mut oferta_params: UpdateOfertaDto = params.into_inner();
    oferta_params.id = path.into_inner();

    let infrastructure = MariaDbRepository::new(state.read().unwrap().db.clone());
    let niveles_infrastructure = MariaDbRepositoryNiveles;

    let application = Update::new(infrastructure, niveles_infrastructure);
    let result = application.execute(oferta_params).await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
