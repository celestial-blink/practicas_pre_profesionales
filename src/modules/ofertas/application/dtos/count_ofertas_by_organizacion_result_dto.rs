#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct CountOfertasByOrganizacionResultDto {
    pub vacantes: rust_decimal::Decimal,
    pub organizacion: String,
    pub id_organizacion: i32,
    pub alias: String,
    pub logo: String,
}
