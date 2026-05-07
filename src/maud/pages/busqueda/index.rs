use actix_web::{
    HttpResponse, get,
    web::{self, Data},
};
use maud::html;

use crate::{
    general_types::State,
    maud::{components::{
        footer::footer,
        head::{HeadProps, head_component}, header::header,
    }, pages::{busqueda::{aside_filters::aside_filters, top_search::top_search}, general::header_items::header_items}},
    modules::ofertas::{
        application::{
            dtos::ofertas_filter_params_dto::OfertasFilterParamsDto, ofertas_filter::OfertasFilter,
        },
        infrastructure::queries::mariadb_query::MariaDbQuery,
    },
};

#[get("/busqueda")]
pub async fn busqueda_view(
    state: Data<State>,
    query: web::Query<OfertasFilterParamsDto>,
) -> HttpResponse {
    // let infrastructure = MariaDbQuery;
    // let oferta_filter = OfertasFilter::new(&infrastructure);
    // let oferta_result = oferta_filter.execute(&state.db, query.into_inner()).await;

    // if oferta_result.is_err() {
    //     return HttpResponse::InternalServerError().body(oferta_result.err().unwrap());
    // }

    // let oferta_result =
    let markup = html!(
            (head_component(HeadProps {
            title: "Busqueda de practicas".to_owned(),
            metadata: None,
            canonical: Some("https://www.practicasperupro.com/busqueda".to_owned()),
            scripts_extra: None,
            css_extra: None,
            include_analytics: true,
            include_ads: true,
        }))
        (header(header_items()))
        section class="py-20 bg-slate-950/50" {
            div class="flex flex-col gap-4 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                br;
                br;
                (top_search())
                div class="grid grid-cols-1 lg:grid-cols-[400px_1fr] gap-4" {
                    aside class="flex-1" {
                        (aside_filters())
                    }
                    main class="flex-1" {
                        "Hola mundo"
                    }
                }
            }
        }
        (footer())
    );

    HttpResponse::Ok().body(markup)
}
