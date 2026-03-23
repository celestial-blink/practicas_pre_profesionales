use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow)]
pub struct Token {
    pub id: i32,
    pub descripcion: String,
    pub token: String,
    pub expira_en: Option<OffsetDateTime>,
    pub estado: i8,
    pub creado_en: Option<OffsetDateTime>,
}
