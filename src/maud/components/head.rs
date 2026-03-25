use maud::{html, Markup, DOCTYPE};

pub fn head_component(title: &str, css_version: Option<String>) -> Markup {

    let css = "/public/css/css.prod.css";

    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" href=(format!("{}{}", css, css_version.clone().unwrap_or_default()));
                link rel="icon" href="/public/images/favicon.ico" type="image/x-icon";
                script src=(format!("/public/js/common.js{}", css_version.unwrap_or_default())) {}
                title {
                    (title)
                }
            }
        }
    }
}
