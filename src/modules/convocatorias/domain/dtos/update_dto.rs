use serde::Deserialize;
use time::PrimitiveDateTime;

use crate::modules::convocatorias::domain::convocatoria::Convocatoria;

#[derive(Deserialize, Debug)]
pub struct UpdateConvocatoriaDto {
    pub id: i32,
    pub titulo: String,
    pub alias: String,
    pub id_organizacion: i32,
    pub nombre_org: String,
    pub logo_org: String,
    pub alias_org: String,
    pub fin_convocatoria: PrimitiveDateTime,
    pub carreras: String,
    pub departamentos: String,
    pub subvenciones: String,
    pub modalidades: String,
    pub nivel_estudios: String,
    pub texto: Option<String>,
    pub finalizan_todos: bool,
    pub estado: i8,
}

impl From<UpdateConvocatoriaDto> for Convocatoria {
    fn from(params: UpdateConvocatoriaDto) -> Self {
        Self {
            id: params.id,
            titulo: params.titulo,
            alias: params.alias,
            id_organizacion: params.id_organizacion,
            nombre_org: params.nombre_org,
            logo_org: params.logo_org,
            alias_org: params.alias_org,
            fin_convocatoria: params.fin_convocatoria,
            carreras: params.carreras,
            departamentos: params.departamentos,
            subvenciones: params.subvenciones,
            modalidades: params.modalidades,
            nivel_estudios: params.nivel_estudios,
            texto: params.texto,
            finalizan_todos: params.finalizan_todos,
            estado: params.estado,
            actualizado_en: None,
            creado_en: None,
        }
    }
}
