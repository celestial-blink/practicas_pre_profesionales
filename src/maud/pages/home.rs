use actix_web::{Result as AwResult, get};
use maud::{Markup, html};

use crate::{config::IS_DEV, helpers, maud::{components::{head::head_component, left_home::left_home, menu_home::{Departamento, FormacionAcademica, menu_home}, right_home::right_home}, layouts::home_layout::home_layout}};

#[get("/")]
pub async fn home_index() -> AwResult<Markup> {

    let css_version = if IS_DEV {
        // time current
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        Some(format!("?t={}", duration.as_secs()))
    } else {
        None
    };

    let departamentos = helpers::t_json::file_to_json::<Vec<Departamento>, _>("./assets/json/departamentos.json");
    let formacion_academica = helpers::t_json::file_to_json::<Vec<FormacionAcademica>, _>("./assets/json/formacion_academica.json");

    let menu = html!(br; (menu_home(departamentos, formacion_academica)));

    Ok(html! {
        (head_component("Practicas Pre y Profesionales Peru", css_version))
        body class="bg-dark-background" {
            (home_layout(left_home(Some(menu)), right_home() ))
        }
    })
}
