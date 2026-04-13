use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    modules::ofertas::{
        application::get_all_by_id_convocatoria::GetAllByIdConvocatoria,
        infrastructure::persistence::mariadb_repository::MariaDbRepository, presentation::router::dtos::generate_texto_by_convocatoria_dto::GenerateTextoByConvocatoriaDto,
    },
};

#[get("/convocatoria/{id}")]
pub async fn get_all_by_id_convocatoria(
    state: web::Data<State>,
    params: web::Path<i32>,
) -> impl Responder {
    let id = params.into_inner();
    let infrastructure = MariaDbRepository::new(state.db.clone());
    let application = GetAllByIdConvocatoria::new(infrastructure);
    let result = application.execute(id).await;



    match result {
        Ok(ofertas) => {
            // TODO: completar
            let result = GenerateTextoByConvocatoriaDto {
                fin_convocatoria: ofertas.fin_convocatoria,
                vacantes: ofertas.vacantes,
                carreras: ofertas.carreras,
                departamentos: ofertas.departamentos,
                subvenciones: ofertas.subvenciones,
                modalidades: ofertas.modalidades,
                nivel_estudios: ofertas.nivel_estudios,
                texto: ofertas.texto,
                finalizan_todos: ofertas.finalizan_todos,
            };

            HttpResponse::Ok().json(result)
        },
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
