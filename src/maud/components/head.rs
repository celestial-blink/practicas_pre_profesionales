use std::{collections::HashMap};

use maud::{DOCTYPE, Markup, html};

use crate::config::IS_DEV;

pub struct HeadProps {
    pub title: String,
    pub metadata: Option<HashMap<&'static str, &'static str>>,
    pub canonical: Option<String>,
    pub scripts_extra: Option<Vec<String>>,
    pub css_extra: Option<Vec<String>>,
    pub include_analytics: bool,
    pub include_ads: bool,
}

pub fn head_component(props: HeadProps) -> Markup {
    let css_version = if IS_DEV {
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        Some(format!("?t={}", duration.as_secs()))
    } else {
        None
    };

    let css = "/public/css/css.prod.css";
    let common_js = format!(
        "/public/js/common{}",
        if css_version.is_some() {
            format!(".js{}", css_version.clone().unwrap())
        } else {
            ".prod.js".to_string()
        }
    );

    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="color-scheme" id="color-scheme" content="dark";
                link rel="stylesheet" href=(format!("{}{}", css, css_version.clone().unwrap_or_default()));
                link rel="icon" href="/public/images/favicon.ico" type="image/x-icon";

                // font iter
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap" rel="stylesheet";

                @if let Some(canonical) = props.canonical {
                    link rel="canonical" href=(canonical);
                }
                @if let Some(metadata) = props.metadata {
                    @for (key, value) in metadata {
                        meta name=(key) content=(value);
                    }
                }
                script src=(common_js) {}
                title {
                    (props.title)
                }
                @if let Some(scripts_extra) = props.scripts_extra {
                    @for script in scripts_extra {
                        script src=(format!("{}{}{}.js", script, if IS_DEV { "" } else { ".prod" }, css_version.clone().unwrap_or_default())) {}
                    }
                }
                @if let Some(css_extra) = props.css_extra {
                    @for css in css_extra {
                        link rel="stylesheet" href=(format!("{}{}{}.css", css, if IS_DEV { "" } else { ".prod" }, css_version.clone().unwrap_or_default())) {}
                    }
                }

                @if props.include_analytics && !IS_DEV {
                    script async src="https://www.googletagmanager.com/gtag/js?id=G-B286837127" {}
                    script {
                        "window.dataLayer = window.dataLayer || [];"
                        "function gtag(){dataLayer.push(arguments);}"
                        "gtag('js', new Date());"
                        "gtag('config', 'G-B286837127');"
                    }
                }
                @if props.include_ads && !IS_DEV {
                    script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-8499382659830077" crossorigin="anonymous" {}
                }
            }
        }
    }
}
