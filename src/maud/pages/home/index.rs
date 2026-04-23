use actix_web::{Result as AwResult, get};
use maud::{Markup, html};

use crate::maud::{
    components::{footer::footer, head::{HeadProps, head_component}},
    pages::home::{guide::guide, header::header, hero::hero, last_convocatoria::{self, last_convocatoria}, meta::meta, top_carreras::{self, top_carrera}},
};



#[get("/")]
pub async fn home_index() -> AwResult<Markup> {
    let top_carreras_props = top_carreras::TopCarreraProps {
        items: vec![
            top_carreras::TopCarreraItem {
                title: "Ingeniería de Sistemas".to_string(),
                total: 10,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                )
            },
            top_carreras::TopCarreraItem {
                title: "Administración".to_string(),
                total: 8,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                )
            },
            top_carreras::TopCarreraItem {
                title: "Contabilidad".to_string(),
                total: 6,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                )
            },
            top_carreras::TopCarreraItem {
                title: "Derecho".to_string(),
                total: 4,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                )
            },
            top_carreras::TopCarreraItem {
                title: "Psicología".to_string(),
                total: 2,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                )
            },
        ],
    };

    let last_convocatoria_props = last_convocatoria::LastConvocatoriaProps {
        items: vec![
            last_convocatoria::LastConvocatoriaItem {
                title: "Ingeniería de Sistemas".to_string(),
                company: "Empresa 1".to_string(),
                location: "Lima".to_string(),
                url: "https://www.practicasperupro.com/".to_string(),
                image: "/public/images/organizaciones/indecopi-20133840533.webp".to_string(),
            },
            last_convocatoria::LastConvocatoriaItem {
                title: "Administración".to_string(),
                company: "Empresa 2".to_string(),
                location: "Lima".to_string(),
                url: "https://www.practicasperupro.com/".to_string(),
                image: "/public/images/organizaciones/indecopi-20133840533.webp".to_string(),
            },
            last_convocatoria::LastConvocatoriaItem {
                title: "Contabilidad".to_string(),
                company: "Empresa 3".to_string(),
                location: "Lima".to_string(),
                url: "https://www.practicasperupro.com/".to_string(),
                image: "/public/images/organizaciones/indecopi-20133840533.webp".to_string(),
            },
            last_convocatoria::LastConvocatoriaItem {
                title: "Derecho".to_string(),
                company: "Empresa 4".to_string(),
                location: "Lima".to_string(),
                url: "https://www.practicasperupro.com/".to_string(),
                image: "/public/images/organizaciones/indecopi-20133840533.webp".to_string(),
            },
            last_convocatoria::LastConvocatoriaItem {
                title: "Psicología".to_string(),
                company: "Empresa 5".to_string(),
                location: "Lima".to_string(),
                url: "https://www.practicasperupro.com/".to_string(),
                image: "/public/images/organizaciones/indecopi-20133840533.webp".to_string(),
            },
        ],
    };

    Ok(html! {
        (head_component(
            HeadProps {
                title: "Practicas Pre y Profesionales Peru".to_string(),
                metadata: Some(meta()),
                canonical: Some("https://www.practicasperupro.com/".to_string()),
                scripts_extra: None,
                css_extra: None,
                include_analytics: true,
                include_ads: true,
            }
        ))
        (header())
        (hero())
        (top_carrera(top_carreras_props))
        (guide())
        (last_convocatoria(last_convocatoria_props))
        (footer())
    })
}
