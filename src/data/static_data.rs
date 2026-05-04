use crate::{
    load_json,
    types::{departamento::Departamento, formacion_academica::FormacionAcademica},
};

pub const DEPARTAMENTOS: std::sync::LazyLock<Vec<Departamento>> = std::sync::LazyLock::new(|| {
    load_json!("../../assets/json/departamentos.json", Vec<Departamento>)
});
pub const FORMACION_ACADEMICAS: std::sync::LazyLock<Vec<FormacionAcademica>> =
    std::sync::LazyLock::new(|| {
        load_json!(
            "../../assets/json/formacion_academica.json",
            Vec<FormacionAcademica>
        )
    });
