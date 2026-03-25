use maud::{html, Markup, DOCTYPE};

pub fn head_component(title: &str, css_version: Option<String>) -> Markup {

    let css = "/public/css/css.prod.css";
    let common_js = format!("/public/js/common{}", if css_version.is_some() { format!(".js{}", css_version.clone().unwrap()) } else { ".prod.js".to_string() });

    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="color-scheme" id="color-scheme" content="dark";
                link rel="stylesheet" href=(format!("{}{}", css, css_version.unwrap_or_default()));
                link rel="icon" href="/public/images/favicon.ico" type="image/x-icon";
                script src=(common_js) {}
                title {
                    (title)
                }
            }
        }
    }
}
