use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, put, web};

use crate::{
    general_types::State,
    modules::pre_ofertas::{
        application::update::Update,
        domain::{dto::update_dto::UpdatePreOfertasDto, pre_ofertas::PreOfertas},
        infrastructure::persistence::mariadb_repository::MariadbRepository,
    },
};

#[put("/{id}")]
pub async fn update(
    state: web::Data<RwLock<State>>,
    path: web::Path<i32>,
    params: web::Json<UpdatePreOfertasDto>,
) -> impl Responder {
    let id = path.into_inner();
    let mut pre_oferta_params: UpdatePreOfertasDto = params.into_inner();
    pre_oferta_params.id = id;
    
    let pre_ofertas: PreOfertas = pre_oferta_params.into();

    let infrastructure = MariadbRepository::new(state.read().unwrap().db.clone());

    let application = Update::new(infrastructure);
    let result = application.execute(pre_ofertas).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
