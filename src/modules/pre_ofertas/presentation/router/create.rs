use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    general_types::State,
    modules::pre_ofertas::{
        application::create::Create,
        domain::{dto::create_dto::CreatePreOfertasDto, pre_ofertas::PreOfertas},
        infrastructure::persistence::mariadb_repository::MariadbRepository,
    },
};

#[post("/")]
pub async fn create(
    state: web::Data<RwLock<State>>,
    params: web::Json<CreatePreOfertasDto>,
) -> impl Responder {
    let pre_oferta_params: CreatePreOfertasDto = params.into_inner();
    let pre_ofertas: PreOfertas = pre_oferta_params.into();

    let infrastructure = MariadbRepository::new(state.read().unwrap().db.clone());

    let application = Create::new(infrastructure);
    let result = application.execute(pre_ofertas).await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
