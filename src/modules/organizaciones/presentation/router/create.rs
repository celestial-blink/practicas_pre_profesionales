use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    general_types::State,
    modules::organizaciones::{
        application::create::Create, domain::dtos::create_dto::CreateParams, infrastructure::{
            mariadb_repository::MariadbRepository, storage::file_storage::FileStorage,
        }
    },
};

use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};



#[derive(MultipartForm)]
pub struct CreateRequest {
    pub params: MpJson<CreateParams>,
    #[multipart(limit = "1MB")]
    pub logo_file: TempFile,
}

#[post("/")]
pub async fn create(
    state: web::Data<State>,
    MultipartForm(params): MultipartForm<CreateRequest>,
) -> impl Responder {
    let logo = params.logo_file;
    let params = params.params.into_inner();


    let organizacion_params = params.into();

    let infrastructure = MariadbRepository::new(state.db.clone());
    let storage_infrastructure = FileStorage;

    let application = Create::new(infrastructure, storage_infrastructure);
    let result = application.execute(organizacion_params, logo).await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
