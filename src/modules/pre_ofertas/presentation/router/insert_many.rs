use actix_web::{HttpResponse, Responder, post, web::{self, Data}};
use serde::Deserialize;

use crate::{general_types::State, modules::pre_ofertas::{application::create_many::CreateMany, domain::pre_ofertas::PreOfertas, infrastructure::mariadb_repository::MariadbRepository}};

#[derive(Deserialize)]
struct PreOfertasRequest {
    titulo: String,
    id_organizacion: i32,
    nombre_organizacion: String,
    modalidad_practicas: u8,
    id_region: u8,
    region: String,
    distrito: String,
    url_oferta: String,
    hash_oferta: String,
    estado: u8,
}

#[post("/insert-many")]
pub async fn insert_many(
    state: Data<State>,
    pre_ofertas: web::Json<Vec<PreOfertasRequest>>,
) -> impl Responder {
    let pre_ofertas = pre_ofertas.into_inner();
    let pre_ofertas = pre_ofertas.iter().map(|pre_oferta| {
        PreOfertas {
            id: 0,
            titulo: pre_oferta.titulo.clone(),
            id_organizacion: pre_oferta.id_organizacion,
            nombre_organizacion: pre_oferta.nombre_organizacion.clone(),
            modalidad_practicas: pre_oferta.modalidad_practicas,
            id_region: pre_oferta.id_region,
            region: pre_oferta.region.clone(),
            distrito: pre_oferta.distrito.clone(),
            url_oferta: pre_oferta.url_oferta.clone(),
            hash_oferta: pre_oferta.hash_oferta.clone(),
            estado: pre_oferta.estado,
            creado_en: None,
        }
    }).collect::<Vec<PreOfertas>>();

    let infrastructure = MariadbRepository::new(state.db.clone());
    let application = CreateMany::new(infrastructure);
    let result = application.execute(pre_ofertas).await;


    match result {
        Ok(_) => HttpResponse::Ok().body("PreOfertas inserted successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
