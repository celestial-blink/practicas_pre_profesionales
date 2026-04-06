use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::ofertas::{
        application::find_by_search::FindBySearch, domain::dtos::search_params::SearchParams,
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[get("/search")]
pub async fn find_by_search(
    state: web::Data<State>,
    params: web::Query<SearchParams>,
) -> impl Responder {
    let search_params = params.into_inner();
    let infrastructure = MariaDbRepository::new(state.db.clone());
    let application = FindBySearch::new(infrastructure);
    let result = application.execute(search_params).await;
    match result {
        Ok(ofertas) => HttpResponse::Ok().json(ofertas),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
