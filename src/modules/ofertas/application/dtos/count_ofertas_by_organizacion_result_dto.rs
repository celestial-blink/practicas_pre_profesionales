#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct CountOfertasByOrganizacionResultDto {
    pub vacantes: rust_decimal::Decimal,
    pub organizacion: String,
    pub alias: String,
    pub logo: String,
}
