use actix_web::{
    Result as AwResult, get,
    web::Data,
};
use maud::{Markup, html};

use crate::{
    general_types::State,
    maud::{
        components::{
            footer::footer,
            head::{HeadProps, head_component},
            header::header,
        },
        pages::{
            general::header_items::header_items,
            home::{
                guide::guide,
                hero::hero,
                last_convocatoria::{self, last_convocatoria},
                meta::meta,
                top::{self, top},
            },
        },
    },
    modules::convocatorias::{
        application::{
            dtos::get_all_actives_params_dto::GetAllActivesParamsDto,
            get_all_actives::GetAllActives,
        },
        infrastructure::queries::mariadb_query::MariaDbQuery,
    },
};

#[get("/")]
pub async fn home_index(state: Data<State>) -> AwResult<Markup> {
    let convocatoria_query_port = MariaDbQuery;
    let get_all_actives = GetAllActives::new(convocatoria_query_port);

    let params = GetAllActivesParamsDto {
        offset: 0,
        limit: 100,
        include_texto: false,
    };

    let convocatorias = get_all_actives.execute(&state.db, params).await;

    let convocatorias = match convocatorias {
        Ok(convocatorias) => convocatorias,
        Err(_) => vec![],
    };

    let top_carreras_props = top::TopProps {
        description: "Las carreras con mayor demanda este mes.",
        link: "/convocatorias",
        link_text: "Ver carreras",
        title: "Explora por Áreas",
        items: vec![
            top::TopItem {
                title: "Ingeniería de Sistemas".to_string(),
                total: 10,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                ),
            },
            top::TopItem {
                title: "Administración".to_string(),
                total: 8,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                ),
            },
            top::TopItem {
                title: "Contabilidad".to_string(),
                total: 6,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                ),
            },
            top::TopItem {
                title: "Derecho".to_string(),
                total: 4,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                ),
            },
            top::TopItem {
                title: "Psicología".to_string(),
                total: 2,
                icon: html! (
                    svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" { }
                        path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" { }
                    }
                ),
            },
        ],
    };

    let last_convocatoria_props = last_convocatoria::LastConvocatoriaProps {
        items: convocatorias
            .into_iter()
            .map(|convocatoria| convocatoria.into())
            .collect(),
    };

    Ok(html! {
        (head_component(
            HeadProps {
                title: "Practicas Pre y Profesionales Perú".to_string(),
                metadata: Some(meta()),
                canonical: Some("https://www.practicasperupro.com/".to_string()),
                scripts_extra: None,
                css_extra: None,
                include_analytics: true,
                include_ads: true,
            }
        ))
        (header(header_items()))
        (hero())
        (top(top_carreras_props))
        (guide())
        (last_convocatoria(last_convocatoria_props))
        (footer())
    })
}
