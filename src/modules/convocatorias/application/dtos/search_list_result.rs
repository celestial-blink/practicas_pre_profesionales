use serde::Serialize;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow, Serialize)]
pub struct SearchListResult {
    pub id: i32,
    pub titulo: String,
    pub nombre_org: String,
    pub estado: i8,
    #[serde(with = "crate::general_types::datetime_no_z")]
    pub creado_en: OffsetDateTime,
}
