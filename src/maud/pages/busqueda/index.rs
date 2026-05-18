use std::sync::RwLock;

use actix_web::{
    HttpResponse, get,
    web::{self, Data},
};
use maud::html;

use crate::{
    general_types::State,
    maud::{
        components::{
            footer::footer,
            head::{HeadProps, head_component},
            header::header,
        },
        pages::{
            busqueda::{
                aside_filters::{AsideFiltersProps, aside_filters},
                main::{MainProps, main},
                top_search::{TopSearchProps, top_search},
            },
            general::header_items::header_items,
        },
    },
    modules::ofertas::{
        application::{
            dtos::ofertas_filter_params_dto::OfertasFilterParamsDto, ofertas_filter::OfertasFilter,
        },
        infrastructure::queries::mariadb_query::MariaDbQuery,
    },
};

#[get("/busqueda")]
pub async fn busqueda_view(
    state: Data<RwLock<State>>,
    query: web::Query<OfertasFilterParamsDto>,
) -> HttpResponse {
    let query_clone = query.clone();
    let query = query.into_inner();
    let departamento_param = query.id_region.unwrap_or(0);
    let search_param = query.search;

    let infrastructure = MariaDbQuery;
    let oferta_filter = OfertasFilter::new(&infrastructure);

    let state = state.read().unwrap();

    let oferta_result = oferta_filter
        .execute(&state.db, query_clone.into_inner())
        .await;

    if oferta_result.is_err() {
        return HttpResponse::InternalServerError().body(oferta_result.err().unwrap());
    }
    let oferta_result = oferta_result.unwrap();

    let markup = html!(
            (head_component(HeadProps {
            title: "Busqueda de practicas".to_owned(),
            metadata: None,
            canonical: Some("https://www.practicasperupro.com/busqueda".to_owned()),
            scripts_extra: Some(vec!["/public/js/pages/busqueda.js".to_owned()]),
            css_extra: None,
            include_analytics: true,
            include_ads: true,
            text_extra: Some(vec![
                format!("<script>const organizaciones = {};</script>", serde_json::to_string(&state.cache.organizaciones).unwrap())
            ])
        }))
        (header(header_items()))
        section class="py-20 bg-slate-950/50" {
            div class="flex flex-col gap-4 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                br;
                br;
                (top_search(TopSearchProps {
                    search: search_param,
                    departamento: departamento_param as u8,
                    organizaciones: &state.cache.organizaciones,
                }))
                div class="grid grid-cols-1 lg:grid-cols-[300px_1fr] gap-4" {
                    aside class="flex-1" {
                        (aside_filters(AsideFiltersProps {
                            organizaciones: &state.cache.organizaciones,
                        }))
                    }
                    main class="flex-1" {
                        (main(MainProps {
                            total_ofertas: oferta_result.total_activas as u32,
                            ofertas: oferta_result.ofertas_activas,
                            ofertas_vencidas: oferta_result.ofertas_vencidas,
                        }))
                    }
                }
            }
        }
        (footer())
    );

    HttpResponse::Ok().body(markup)
}
