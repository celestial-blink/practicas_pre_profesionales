use crate::{
    load_json,
    maud::components::header::MenuItem,
    types::{departamento::Departamento, formacion_academica::FormacionAcademica},
};

pub fn header_items() -> Vec<MenuItem> {
    let departamentos = load_json!(
        "../../../../assets/json/departamentos.json",
        Vec<Departamento>
    );
    let formacion_academicas = load_json!(
        "../../../../assets/json/formacion_academica.json",
        Vec<FormacionAcademica>
    );

    let departamentos_sub_menu: Vec<MenuItem> = departamentos
        .into_iter()
        .map(|item| MenuItem {
            title: item.nombre,
            url: format!("/departamentos/{}", item.alias),
            is_call_to_action: false,
            sub_menu: None,
            target: None,
        })
        .collect();

    let formacion_academicas_sub_menu: Vec<MenuItem> = formacion_academicas
        .into_iter()
        .map(|item| MenuItem {
            title: item.nombre,
            url: format!("/formacion/{}", item.alias),
            is_call_to_action: false,
            sub_menu: None,
            target: None,
        })
        .collect();

    vec![
        MenuItem {
            title: "Departamentos".to_string(),
            url: "/departamentos".to_string(),
            is_call_to_action: false,
            sub_menu: Some(departamentos_sub_menu),
            target: None,
        },
        MenuItem {
            title: "Formación".to_string(),
            url: "/formacion".to_string(),
            is_call_to_action: false,
            sub_menu: Some(formacion_academicas_sub_menu),
            target: None,
        },
        MenuItem {
            title: "Organizaciones".to_string(),
            url: "/organizaciones".to_string(),
            is_call_to_action: false,
            sub_menu: None,
            target: None,
        },
        MenuItem {
            title: "Buscar Prácticas".to_string(),
            url: "#publicar".to_string(),
            is_call_to_action: true,
            sub_menu: None,
            target: None,
        },
    ]
}
