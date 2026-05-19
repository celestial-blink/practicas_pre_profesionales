use crate::data::static_data::DEPARTAMENTOS;
use crate::types::departamento::Departamento;

pub fn get_departamento_by_id(id: u32) -> Option<Departamento> {
    for departamento in DEPARTAMENTOS.iter() {
        if departamento.id == id as i32 {
            return Some(departamento.clone());
        }
    }
    None
}
