use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::ofertas::{
        application::{dtos::search_params::SearchParams, find_by_search::FindBySearch},
        infrastructure::queries::mariadb_query::MariaDbQuery,
    },
};

#[get("/search")]
pub async fn find_by_search(
    state: web::Data<RwLock<State>>,
    params: web::Query<SearchParams>,
) -> impl Responder {
    let search_params = params.into_inner();
    let infrastructure = MariaDbQuery;
    let application = FindBySearch::new(infrastructure);
    let result = application
        .execute(&state.read().unwrap().db, search_params)
        .await;
    match result {
        Ok(ofertas) => HttpResponse::Ok().json(ofertas),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
