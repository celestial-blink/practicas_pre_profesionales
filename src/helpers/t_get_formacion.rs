use crate::data::static_data::FORMACION_ACADEMICAS;
use crate::types::formacion_academica::FormacionAcademica;

pub fn get_formacion_by_alias(alias: &str) -> Option<FormacionAcademica> {
    for formacion in FORMACION_ACADEMICAS.iter() {
        if formacion.alias.to_lowercase() == alias.to_lowercase() {
            return Some(formacion.clone());
        }
    }
    None
}
