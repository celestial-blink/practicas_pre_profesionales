use std::sync::RwLock;

use actix_web::{get, web::Data};
use maud::{Markup, html};

use crate::{
    general_types::State,
    maud::{
        components::{
            footer::footer,
            head::{HeadProps, head_component},
            header::header,
        },
        pages::{general::header_items::header_items, organizaciones::hero},
    },
    modules::ofertas::{
        application::get_count_ofertas_by_organizacion::GetCountOfertasByOrganizacion,
        infrastructure::queries::mariadb_query::MariaDbQuery,
    },
};

#[get("/organizaciones")]
pub async fn organizaciones_view(state: Data<RwLock<State>>) -> Markup {
    let infrastructure = MariaDbQuery {};
    let get_count_ofertas = GetCountOfertasByOrganizacion::new(infrastructure);

    let state = state.read().unwrap();

    let ofertas = get_count_ofertas.execute(&state.db).await;

    let organizaciones = match ofertas {
        Ok(ofertas) => ofertas,
        Err(_) => vec![],
    };

    html! {
        (head_component(HeadProps {
            title: "Lista de organizaciones".to_owned(),
            metadata: None,
            canonical: Some("https://www.practicasperupro.com/organizaciones".to_owned()),
            scripts_extra: None,
            css_extra: None,
            include_analytics: true,
            include_ads: true,
            text_extra: None,
        }))
        (header(header_items()))
        (hero::hero())
        section class="py-20 bg-slate-950/50" {
            div class="flex flex-col gap-4 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                main class="flex-1" {
                    div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-12" {
                        @for (index, organizacion) in organizaciones.iter().enumerate() {
                            a href=(format!("/organizacion/{}", organizacion.alias)) class="bg-theme-glass p-6 rounded-2xl flex flex-col relative hover:-translate-y-1 hover:bg-rose-500/10 hover:outline-2 outline-rose-500/30 outline-offset-2 transition-all duration-300" target="_blank" {
                                @if index < 3 {
                                    div class="absolute top-4 right-4 w-3 h-3 bg-purple-500 rounded-full animate-pulse" { }
                                }
                                h3 class="font-bold text-lg" {
                                    (organizacion.organizacion)
                                }
                                p class="text-blue-400 text-sm font-semibold" {
                                    (format!("{} vacantes", organizacion.vacantes))
                                }
                            }
                        }
                    }
                }
            }
        }
        (footer())
    }
}
