use std::sync::RwLock;

use actix_web::{HttpResponse, get, web::Data};
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
                meta::meta,
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
    query: serde_qs::actix::QsQuery<OfertasFilterParamsDto>,
) -> HttpResponse {
    let query_clone = query.clone();
    let limit = 2;
    let infrastructure = MariaDbQuery::new();
    let oferta_filter = OfertasFilter::new(&infrastructure);

    let state = state.read().unwrap();

    let oferta_result = oferta_filter
        .execute(&state.db, query_clone.clone().into_inner(), limit)
        .await;

    if oferta_result.is_err() {
        return HttpResponse::InternalServerError().body(oferta_result.err().unwrap());
    }
    let oferta_result = oferta_result.unwrap();

    let markup = html!(
            (head_component(HeadProps {
            title: "Busqueda de practicas pre y profesionales".to_owned(),
            metadata: Some(meta()),
            alternative_metadata: None,
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
                    query_params: &query_clone,
                }))
                div class="grid grid-cols-1 lg:grid-cols-[300px_1fr] gap-4" {
                    aside class="flex-1" {
                        (aside_filters(AsideFiltersProps {
                            organizaciones: &state.cache.organizaciones,
                            query_params: &query_clone,
                        }))
                    }
                    main class="flex-1" {
                        (main(MainProps {
                            total_ofertas: oferta_result.total_activas as u32,
                            ofertas: oferta_result.ofertas_activas,
                            ofertas_vencidas: oferta_result.ofertas_vencidas,
                            per_page: limit as u32,
                            query_params: &query_clone,
                            limit,
                            organizaciones: &state.cache.organizaciones,
                        }))
                    }
                }
            }
        }
        (footer())
    );

    HttpResponse::Ok().body(markup)
}
