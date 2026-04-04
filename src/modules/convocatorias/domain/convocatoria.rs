use serde::Serialize;
use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, FromRow, Serialize)]
pub struct Convocatoria {
    pub id: i32,
    pub titulo: String,
    pub alias: String,
    pub id_organizacion: i32,
    pub nombre_org: String,
    pub logo_org: String,
    pub alias_org: String,
    pub fin_convocatoria: PrimitiveDateTime,
    pub carreras: String,
    pub departamentos: String,
    pub subvenciones: String,
    pub modalidades: String,
    pub nivel_estudios: String,
    pub texto: Option<String>,
    pub finalizan_todos: bool,
    pub estado: i8,
    pub actualizado_en: Option<PrimitiveDateTime>,
    pub creado_en: Option<OffsetDateTime>,
}
