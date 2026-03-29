use serde::Deserialize;

use crate::modules::organizaciones::domain::organizacion::Organizacion;

#[derive(Deserialize, Debug)]
pub struct CreateParams {
    pub razon_social: String,
    pub nombre_comercial: String,
    pub alias: String,
    pub ruc: String,
    pub logo: String,
    pub tipo: i8,
    pub estado: i8,
}


impl From<CreateParams> for Organizacion {
    fn from(params: CreateParams) -> Self {
        Self {
            id: 0,
            razon_social: params.razon_social,
            nombre_comercial: params.nombre_comercial,
            alias: params.alias,
            ruc: params.ruc,
            logo: params.logo,
            tipo: params.tipo,
            estado: params.estado,
            creado_en: None,
        }
    }
}
