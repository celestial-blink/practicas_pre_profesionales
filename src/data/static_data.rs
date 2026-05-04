use crate::{load_json, types::departamento::Departamento};

pub const DEPARTAMENTOS: std::sync::LazyLock<Vec<Departamento>> = std::sync::LazyLock::new(|| load_json!("../../assets/json/departamentos.json", Vec<Departamento>));
