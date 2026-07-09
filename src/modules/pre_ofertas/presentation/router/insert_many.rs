use std::sync::RwLock;

use actix_web::{
    HttpResponse, Responder, post,
    web::{self, Data},
};

use crate::{
    general_types::State,
    modules::pre_ofertas::{
        application::create_many::CreateMany,
        domain::{dto::create_dto::CreatePreOfertasDto, pre_ofertas::PreOfertas},
        infrastructure::persistence::mariadb_repository::MariadbRepository,
    },
};

#[post("/insert-many")]
pub async fn insert_many(
    state: Data<RwLock<State>>,
    pre_ofertas: web::Json<Vec<CreatePreOfertasDto>>,
) -> impl Responder {
    let pre_ofertas = pre_ofertas.into_inner();
    let pre_ofertas = pre_ofertas
        .into_iter()
        .map(Into::into)
        .collect::<Vec<PreOfertas>>();

    let infrastructure = MariadbRepository::new(state.read().unwrap().db.clone());
    let application = CreateMany::new(infrastructure);
    let result = application.execute(pre_ofertas).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("PreOfertas inserted successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
