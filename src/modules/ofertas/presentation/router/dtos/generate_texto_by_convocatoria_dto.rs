use serde::Serialize;
use time::{PrimitiveDateTime};

#[derive(Debug, Serialize)]
pub struct GenerateTextoByConvocatoriaDto {
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
}
