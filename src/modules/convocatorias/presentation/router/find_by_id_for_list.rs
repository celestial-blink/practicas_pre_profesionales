use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::convocatorias::{
        application::find_by_id_for_list::FindByIdForList,
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[get("/{id}/search-list")]
pub async fn find_by_id_for_list(
    state: web::Data<State>,
    params: web::Path<i32>,
) -> impl Responder {
    let id = params.into_inner();
    let infrastructure = MariaDbRepository::new(state.db.clone());
    let application = FindByIdForList::new(infrastructure);
    let result = application.execute(id).await;

    match result {
        Ok(convocatoria) => HttpResponse::Ok().json(convocatoria),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
