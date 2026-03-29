use actix_web::{HttpResponse, Responder, put, web};

use crate::{
    general_types::State,
    modules::organizaciones::{
        application::update::Update,
        domain::{dtos::create_dto::CreateParams, organizacion::Organizacion},
        infrastructure::{
            mariadb_repository::MariadbRepository, storage::file_storage::FileStorage,
        },
    },
};

use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};

#[derive(MultipartForm)]
pub struct UpdateRequest {
    pub params: MpJson<CreateParams>,
    pub logo_file: Option<TempFile>,
}

#[put("/{id}")]
pub async fn update(
    state: web::Data<State>,
    path: web::Path<i32>,
    MultipartForm(params): MultipartForm<UpdateRequest>,
) -> impl Responder {
    let logo = params.logo_file;
    let params = params.params.into_inner();

    let id = path.into_inner();

    let mut organizacion_params: Organizacion = params.into();
    organizacion_params.id = id;


    let infrastructure = MariadbRepository::new(state.db.clone());
    let storage_infrastructure = FileStorage;

    let application = Update::new(infrastructure, storage_infrastructure);
    let result = application.execute(organizacion_params, logo).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
