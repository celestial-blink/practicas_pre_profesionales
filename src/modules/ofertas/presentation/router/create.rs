use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    general_types::State,
    modules::oferta_niveles::infrastructure::persitense::mariadb_repository::MariaDbRepository as MariaDbNivelesRepository,
    modules::ofertas::{
        application::create::Create, domain::dtos::create_dto::CreateOfertaDto,
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[post("/")]
pub async fn create(state: web::Data<State>, params: web::Json<CreateOfertaDto>) -> impl Responder {
    let db = state.db.clone();

    let oferta_params: CreateOfertaDto = params.into_inner();

    let infrastructure = MariaDbRepository::new(db.clone());
    let niveles_infrastructure = MariaDbNivelesRepository::new(db.clone());

    let application = Create::new(infrastructure, niveles_infrastructure);
    let result = application.execute(oferta_params).await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
