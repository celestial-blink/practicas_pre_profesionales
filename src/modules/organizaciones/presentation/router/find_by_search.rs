use std::sync::RwLock;

use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::organizaciones::{
        application::find_by_search::FindBySearch, domain::dto::SearchParams,
        infrastructure::mariadb_query_repository::MariadbQueryRepository,
    },
};

#[get("/search")]
pub async fn find_by_search(
    state: web::Data<RwLock<State>>,
    params: web::Query<SearchParams>,
) -> impl Responder {
    let search_params = params.into_inner();
    let infrastructure = MariadbQueryRepository;
    let application = FindBySearch::new(infrastructure);
    let state = state.read().unwrap();
    let result = application.execute(&state.db.clone(), search_params).await;
    match result {
        Ok(organizaciones) => HttpResponse::Ok().json(organizaciones),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
