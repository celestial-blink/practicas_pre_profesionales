use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchParams {
    pub search: Option<String>,
    pub tipo: Option<i8>,
    pub estado: Option<i8>,
    pub limit: i32,
    pub offset: i32,
}
