use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::pre_ofertas::{
        application::{dto::search_params::SearchParams, find_by_search::FindBySearch},
        infrastructure::query::mariadb_query_repository::PreOfertasQueryRepository,
    },
};

#[get("/search")]
pub async fn find_by_search(
    state: web::Data<RwLock<State>>,
    params: web::Query<SearchParams>,
) -> impl Responder {
    let search_params = params.into_inner();
    let infrastructure = PreOfertasQueryRepository;
    let application = FindBySearch::new(infrastructure);
    let result = application
        .execute(&state.read().unwrap().db.clone(), search_params)
        .await;
    match result {
        Ok(pre_ofertas) => HttpResponse::Ok().json(pre_ofertas),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
