use serde::Serialize;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow, Serialize)]
pub struct PreOfertas {
    pub id: i32,
    pub titulo: String,
    pub id_organizacion: i32,
    pub nombre_organizacion: String,
    pub modalidad_practicas: u8,
    pub id_region: u8,
    pub region: String,
    pub distrito: String,
    pub url_oferta: String,
    pub hash_oferta: String,
    pub estado: u8,
    #[serde(with = "crate::general_types::datetime_no_z_option")]
    pub creado_en: Option<OffsetDateTime>,
}
