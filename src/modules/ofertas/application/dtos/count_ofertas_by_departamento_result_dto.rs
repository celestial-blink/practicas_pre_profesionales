#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct CountOfertasByDepartamentoResultDto {
    pub vacantes: rust_decimal::Decimal,
    pub departamento: String,
    pub id_departamento: i32,
}
