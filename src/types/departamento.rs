#[derive(serde::Deserialize, Clone)]
pub struct Departamento {
    pub id: i32,
    pub nombre: String,
    pub alias: String,
}
