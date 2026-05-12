use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    general_types::State,
    modules::convocatorias::{
        application::create::Create,
        domain::{convocatoria::Convocatoria, dtos::create_dto::CreateConvocatoriaDto},
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[post("/")]
pub async fn create(
    state: web::Data<RwLock<State>>,
    params: web::Json<CreateConvocatoriaDto>,
) -> impl Responder {
    let convocatoria_params: CreateConvocatoriaDto = params.into_inner();
    let convocatoria_params: Convocatoria = convocatoria_params.into();

    let infrastructure = MariaDbRepository::new(state.read().unwrap().db.clone());

    let application = Create::new(infrastructure);
    let result = application.execute(convocatoria_params).await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
