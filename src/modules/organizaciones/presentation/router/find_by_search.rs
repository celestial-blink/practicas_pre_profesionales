use actix_web::{HttpResponse, Responder, get, web};

use crate::{general_types::State, modules::organizaciones::{application::find_by_search::FindBySearch, domain::dto::SearchParams, infrastructure::mariadb_repository::MariadbRepository}};

#[get("/find-by-search")]
pub async fn find_by_search(
    state: web::Data<State>,
    params: web::Query<SearchParams>,
) -> impl Responder {
    let search_params = params.into_inner();
    let infrastructure = MariadbRepository::new(state.db.clone());
    let application = FindBySearch::new(infrastructure);
    let result = application.execute(search_params).await;
    match result {
        Ok(organizaciones) => HttpResponse::Ok().json(organizaciones),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
