use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

use crate::{
    general_types::State,
    modules::organizaciones::{
        application::create::Create, domain::organizacion::Organizacion, infrastructure::{
            mariadb_repository::MariadbRepository, storage::file_storage::FileStorage,
        }
    },
};

use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};

#[derive(Deserialize, Debug)]
pub struct CreateParams {
    pub razon_social: String,
    pub nombre_comercial: String,
    pub alias: String,
    pub ruc: String,
    pub logo: String,
    pub tipo: i8,
    pub estado: i8,
}

#[derive(MultipartForm)]
pub struct CreateRequest {
    pub params: MpJson<CreateParams>,
    pub logo_file: TempFile,
}

#[post("/create")]
pub async fn create(
    state: web::Data<State>,
    MultipartForm(params): MultipartForm<CreateRequest>,
) -> impl Responder {
    let logo = params.logo_file;
    let params = params.params.into_inner();


    let organizacion_params = Organizacion {
        id: 0,
        razon_social: params.razon_social,
        nombre_comercial: params.nombre_comercial,
        alias: params.alias,
        ruc: params.ruc,
        logo: params.logo,
        tipo: params.tipo,
        estado: params.estado,
        creado_en: None,
    };

    let infrastructure = MariadbRepository::new(state.db.clone());
    let storage_infrastructure = FileStorage;

    let application = Create::new(infrastructure, storage_infrastructure);
    let result = application.execute(organizacion_params, logo).await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
