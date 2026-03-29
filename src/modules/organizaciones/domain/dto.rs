use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchParams {
    pub search: Option<String>,
    pub tipo: Option<i8>,
    pub estado: Option<i8>,
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default = "default_offset")]
    pub offset: i32,
}

fn default_limit() -> i32 {
    10
}

fn default_offset() -> i32 {
    0
}
