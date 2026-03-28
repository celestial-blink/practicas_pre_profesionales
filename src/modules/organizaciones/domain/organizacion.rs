use sqlx::prelude::FromRow;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(FromRow, Serialize)]
pub struct Organizacion {
    pub id: i32,
    pub razon_social: String,
    pub nombre_comercial: String,
    pub alias: String,
    pub ruc: String,
    pub logo: String,
    pub tipo: i8,
    pub estado: i8,
    pub creado_en: Option<OffsetDateTime>,
}
