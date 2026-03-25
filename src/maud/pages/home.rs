use actix_web::{Result as AwResult, get};
use maud::{Markup, html};

use crate::{config::IS_DEV, maud::{components::{head::head_component, left_home::left_home, right_home::right_home}, layouts::home_layout::home_layout}};

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

    Ok(html! {
        (head_component("Practicas Pre y Profesionales Peru", css_version))
        body class="bg-dark-background" {
            (home_layout(left_home(None), right_home() ))
        }
    })
}
