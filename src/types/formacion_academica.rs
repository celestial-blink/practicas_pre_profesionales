#[derive(serde::Deserialize)]
pub struct FormacionAcademica {
    pub id: i32,
    pub nombre: String,
    pub alias: String,
}
