use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    general_types::State,
    modules::ofertas::{
        application::create::Create,
        domain::{dtos::create_dto::CreateOfertaDto, oferta::Oferta},
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[post("/")]
pub async fn create(state: web::Data<State>, params: web::Json<CreateOfertaDto>) -> impl Responder {
    let oferta_params: CreateOfertaDto = params.into_inner();
    let oferta_params: Oferta = oferta_params.into();

    let infrastructure = MariaDbRepository::new(state.db.clone());

    let application = Create::new(infrastructure);
    let result = application.execute(oferta_params).await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
