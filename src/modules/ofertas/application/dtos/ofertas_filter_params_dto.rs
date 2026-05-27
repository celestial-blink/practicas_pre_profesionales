use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OfertasFilterParamsDto {
    #[serde(default = "get_default_offset")]
    pub offset: i32,
    pub search: Option<String>,
    pub id_region: Option<i8>,
    pub id_organizacion: Option<Vec<i32>>,
    pub modalidad_practicas: Option<i8>,
    pub niveles: Option<Vec<i8>>,
}

fn get_default_offset() -> i32 {
    0
}
