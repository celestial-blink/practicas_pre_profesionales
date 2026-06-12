use actix_web::{HttpResponse, Responder};
use maud::html;

use crate::maud::{
    components::{
        head::{HeadProps, head_component},
        header::header,
    },
    pages::{
        general::header_items::header_items,
        not_found::component::{NotFoundComponentProps, not_found_component},
    },
};

pub async fn not_found_view() -> impl Responder {
    let html = html! {
        (head_component(HeadProps {
            title: "Página no encontrada".to_string(),
            metadata: None,
            canonical: Some("https://practicasperu.com/not_found/".to_string()),
            scripts_extra: None,
            css_extra: None,
            include_analytics: true,
            include_ads: true,
            text_extra: None,
        }))
        br;
        br;
        br;
        (not_found_component(NotFoundComponentProps {
            title: "Página no encontrada",
            description: "La página que buscas no existe o ha sido movida.",
        }))
        (header(header_items()))
    };

    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(html.into_string())
}
