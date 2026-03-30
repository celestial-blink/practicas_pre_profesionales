use serde::Serialize;
use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, FromRow, Serialize)]
pub struct Oferta {
    pub id: i32,
    pub titulo: String,
    pub alias: String,
    pub id_organizacion: i32,
    pub nombre_org: String,
    pub modalidad_practicas: i8,
    pub vacantes: i16,
    pub subvencion: f64,
    pub fecha_fin_oferta: PrimitiveDateTime,
    pub formacion: String,
    pub funciones: String,
    pub lugar_practicas: String,
    pub como_postular: String,
    pub bases: String,
    pub extra_info: String,
    pub id_region: i8,
    pub region: String,
    pub distrito: String,
    pub estado: i8,
    pub creado_en: Option<OffsetDateTime>,
}
