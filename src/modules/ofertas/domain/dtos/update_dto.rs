use serde::Deserialize;
use time::PrimitiveDateTime;

use crate::modules::ofertas::domain::oferta::Oferta;

#[derive(Deserialize, Debug)]
pub struct UpdateOfertaDto {
    pub id: i32,
    pub titulo: String,
    pub alias: String,
    pub id_organizacion: i32,
    pub nombre_org: String,
    pub modalidad_practicas: i8,
    pub vacantes: i16,
    pub subvencion: f64,
    pub fecha_fin_oferta: PrimitiveDateTime,
    pub formacion: String,
    pub funciones: String,
    pub lugar_practicas: String,
    pub como_postular: String,
    pub bases: String,
    pub extra_info: String,
    pub id_region: i8,
    pub region: String,
    pub distrito: String,
    pub estado: i8,
}

impl From<UpdateOfertaDto> for Oferta {
    fn from(params: UpdateOfertaDto) -> Self {
        Self {
            id: params.id,
            titulo: params.titulo,
            alias: params.alias,
            id_organizacion: params.id_organizacion,
            nombre_org: params.nombre_org,
            modalidad_practicas: params.modalidad_practicas,
            vacantes: params.vacantes,
            subvencion: params.subvencion,
            fecha_fin_oferta: params.fecha_fin_oferta,
            formacion: params.formacion,
            funciones: params.funciones,
            lugar_practicas: params.lugar_practicas,
            como_postular: params.como_postular,
            bases: params.bases,
            extra_info: params.extra_info,
            id_region: params.id_region,
            region: params.region,
            distrito: params.distrito,
            estado: params.estado,
            creado_en: None,
        }
    }
}
