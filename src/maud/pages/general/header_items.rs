use crate::{
    config::DOMAIN,
    data::static_data::{DEPARTAMENTOS, FORMACION_ACADEMICAS},
    maud::components::header::MenuItem,
};

pub fn header_items() -> Vec<MenuItem> {
    let departamentos_sub_menu: Vec<MenuItem> = DEPARTAMENTOS
        .clone()
        .into_iter()
        .map(|item| MenuItem {
            title: item.nombre,
            url: format!("{}/departamento/{}", DOMAIN, item.alias),
            is_call_to_action: false,
            sub_menu: None,
            target: None,
        })
        .collect();

    let formacion_academicas_sub_menu: Vec<MenuItem> = FORMACION_ACADEMICAS
        .clone()
        .into_iter()
        .map(|item| MenuItem {
            title: item.nombre,
            url: format!("{}/formacion/{}", DOMAIN, item.alias),
            is_call_to_action: false,
            sub_menu: None,
            target: None,
        })
        .collect();

    vec![
        MenuItem {
            title: "Departamentos".to_string(),
            url: format!("{}/departamentos", DOMAIN),
            is_call_to_action: false,
            sub_menu: Some(departamentos_sub_menu),
            target: None,
        },
        MenuItem {
            title: "Formación".to_string(),
            url: format!("{}/formacion", DOMAIN),
            is_call_to_action: false,
            sub_menu: Some(formacion_academicas_sub_menu),
            target: None,
        },
        MenuItem {
            title: "Organizaciones".to_string(),
            url: format!("{}/organizaciones", DOMAIN),
            is_call_to_action: false,
            sub_menu: None,
            target: None,
        },
        MenuItem {
            title: "Buscar Prácticas".to_string(),
            url: format!("{}/busqueda", DOMAIN),
            is_call_to_action: true,
            sub_menu: None,
            target: None,
        },
    ]
}
