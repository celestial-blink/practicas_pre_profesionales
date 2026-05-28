use crate::general_types::default_on_error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct OfertasFilterParamsDto {
    #[serde(default = "get_default_offset")]
    pub offset: i32,
    #[serde(default, deserialize_with = "default_on_error")]
    pub search: Option<String>,
    #[serde(default, deserialize_with = "default_on_error")]
    pub id_region: Option<i8>,
    #[serde(default, deserialize_with = "default_on_error")]
    pub id_organizacion: Option<Vec<i32>>,
    #[serde(default, deserialize_with = "default_on_error")]
    pub modalidad_practicas: Option<i8>,
    #[serde(default, deserialize_with = "default_on_error")]
    pub niveles: Option<Vec<i8>>,
}

fn get_default_offset() -> i32 {
    0
}
