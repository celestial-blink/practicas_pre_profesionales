use serde::Serialize;
use sqlx::prelude::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, FromRow, Serialize)]
pub struct OfertaWithNivelesDto {
    pub id: i32,
    pub id_convocatoria: Option<i32>,
    pub titulo: String,
    pub alias: String,
    pub id_organizacion: i32,
    pub nombre_org: String,
    pub logo_org: String,
    pub alias_org: String,
    pub modalidad_practicas: i8,
    pub vacantes: i16,
    pub subvencion: rust_decimal::Decimal,
    #[serde(with = "crate::general_types::datetime_format")]
    pub fecha_fin_oferta: PrimitiveDateTime,
    pub formacion: String,
    pub carreras: String,
    pub funciones: Option<String>,
    pub lugar_practicas: Option<String>,
    pub como_postular: Option<String>,
    pub bases: Option<String>,
    pub extra_info: Option<String>,
    pub id_region: i8,
    pub region: String,
    pub distrito: String,
    pub niveles: String,
    pub niveles_data: Option<String>,
    pub estado: i8,
    pub creado_en: Option<OffsetDateTime>,
}
