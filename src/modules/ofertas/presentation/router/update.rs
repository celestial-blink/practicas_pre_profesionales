use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    general_types::State,
    modules::ofertas::{
        application::update::Update,
        domain::{
            dtos::update_dto::UpdateOfertaDto,
            oferta::Oferta,
        },
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[post("/")]
pub async fn update(state: web::Data<State>, params: web::Json<UpdateOfertaDto>) -> impl Responder {
    let oferta_params: UpdateOfertaDto = params.into_inner();
    let oferta_params: Oferta = oferta_params.into();

    let infrastructure = MariaDbRepository::new(state.db.clone());

    let application = Update::new(infrastructure);
    let result = application.execute(oferta_params).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
