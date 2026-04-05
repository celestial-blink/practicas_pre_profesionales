use rust_decimal::Decimal;
use serde::Deserialize;
use time::PrimitiveDateTime;

use crate::modules::ofertas::domain::oferta::Oferta;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateOfertaDto {
    pub id_convocatoria: Option<i32>,
    pub titulo: String,
    pub alias: String,
    pub id_organizacion: i32,
    pub nombre_org: String,
    pub logo_org: String,
    pub alias_org: String,
    pub modalidad_practicas: i8,
    pub vacantes: i16,
    pub subvencion: Decimal,
    #[serde(with = "crate::general_types::datetime_format")]
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
    pub niveles: Vec<i8>,
    pub estado: i8,
}

impl From<CreateOfertaDto> for Oferta {
    fn from(params: CreateOfertaDto) -> Self {
        Self {
            id: 0,
            id_convocatoria: params.id_convocatoria,
            titulo: params.titulo,
            alias: params.alias,
            id_organizacion: params.id_organizacion,
            nombre_org: params.nombre_org,
            logo_org: params.logo_org,
            alias_org: params.alias_org,
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
            niveles: params.niveles.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
            estado: params.estado,
            creado_en: None,
        }
    }
}
