use actix_web::{Result as AwResult, get};
use maud::{Markup, html};

use crate::{
    helpers,
    maud::{
        components::{
            head::{HeadProps, head_component},
            left_home::left_home,
            menu_home::menu_home,
            right_home::right_home,
        },
        layouts::home_layout::home_layout,
    },
    types::{departamento::Departamento, formacion_academica::FormacionAcademica},
};

#[get("/practicas-peru")]
pub async fn page_filters() -> AwResult<Markup> {
    let departamentos =
        helpers::t_json::file_to_json::<Vec<Departamento>, _>("./assets/json/departamentos.json");
    let formacion_academica = helpers::t_json::file_to_json::<Vec<FormacionAcademica>, _>(
        "./assets/json/formacion_academica.json",
    );

    let menu = html!(br; (menu_home(departamentos, formacion_academica)));

    Ok(html! {
        (head_component(HeadProps {
            title: "Practicas Pre y Profesionales Peru".to_string(),
            metadata: None,
            canonical: Some("https://www.practicasperupro.com/practicas-peru".to_string()),
            scripts_extra: None,
            css_extra: None,
            include_analytics: true,
            include_ads: true,
        }))
        body class="bg-dark-background" {
            (home_layout(left_home(Some(menu)), right_home() ))
        }
    })
}
