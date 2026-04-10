use actix_web::{HttpResponse, Responder, put, web};

use crate::{
    general_types::State,
    modules::convocatorias::{
        application::update::Update,
        domain::{
            convocatoria::Convocatoria,
            dtos::update_dto::UpdateConvocatoriaDto,
        },
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
    },
};

#[put("/{id}")]
pub async fn update(state: web::Data<State>, params: web::Json<UpdateConvocatoriaDto>) -> impl Responder {
    let convocatoria_params: UpdateConvocatoriaDto = params.into_inner();
    let convocatoria_params: Convocatoria = convocatoria_params.into();

    let infrastructure = MariaDbRepository::new(state.db.clone());

    let application = Update::new(infrastructure);
    let result = application.execute(convocatoria_params).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
