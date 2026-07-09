use serde::Deserialize;

use crate::modules::pre_ofertas::domain::pre_ofertas::PreOfertas;

#[derive(Deserialize, Debug)]
pub struct UpdatePreOfertasDto {
    pub id: i32,
    pub titulo: String,
    pub id_organizacion: i32,
    pub nombre_organizacion: String,
    pub modalidad_practicas: u8,
    pub id_region: u8,
    pub region: String,
    pub distrito: String,
    pub url_oferta: String,
    pub hash_oferta: String,
    pub estado: u8,
}

impl From<UpdatePreOfertasDto> for PreOfertas {
    fn from(params: UpdatePreOfertasDto) -> Self {
        Self {
            id: params.id,
            titulo: params.titulo,
            id_organizacion: params.id_organizacion,
            nombre_organizacion: params.nombre_organizacion,
            modalidad_practicas: params.modalidad_practicas,
            id_region: params.id_region,
            region: params.region,
            distrito: params.distrito,
            url_oferta: params.url_oferta,
            hash_oferta: params.hash_oferta,
            estado: params.estado,
            creado_en: None,
        }
    }
}
