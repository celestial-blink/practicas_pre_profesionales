use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::convocatorias::{
        application::{
            dtos::search_params::SearchParams, find_by_search_for_list::FindBySearchForList,
        },
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[get("/search-list")]
pub async fn find_by_search_for_list(
    state: web::Data<RwLock<State>>,
    params: web::Query<SearchParams>,
) -> impl Responder {
    let search_params = params.into_inner();
    let infrastructure = MariaDbRepository::new(state.read().unwrap().db.clone());
    let application = FindBySearchForList::new(infrastructure);
    let result = application.execute(search_params).await;
    match result {
        Ok(convocatorias) => HttpResponse::Ok().json(convocatorias),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
