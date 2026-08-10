use serde::Serialize;
use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, FromRow, Serialize)]
pub struct SearchResult {
    pub id: i32,
    pub titulo: String,
    pub alias: String,
    pub id_organizacion: i32,
    pub nombre_org: String,
    pub logo_org: String,
    pub alias_org: String,
    #[serde(with = "crate::general_types::datetime_format")]
    pub fin_convocatoria: PrimitiveDateTime,
    pub vacantes: i32,
    pub carreras: String,
    pub departamentos: String,
    pub subvenciones: String,
    pub modalidades: String,
    pub nivel_estudios: String,
    pub texto: Option<String>,
    pub finalizan_todos: bool,
    pub estado: i8,
    #[serde(with = "crate::general_types::datetime_format_option")]
    pub actualizado_en: Option<PrimitiveDateTime>,
    #[serde(with = "crate::general_types::datetime_no_z_option")]
    pub creado_en: Option<OffsetDateTime>,
    pub total: i32,
}
