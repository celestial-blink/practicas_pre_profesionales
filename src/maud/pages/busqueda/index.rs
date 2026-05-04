use actix_web::{
    HttpResponse, get,
    web::{self, Data},
};
use maud::html;

use crate::{
    general_types::State,
    maud::components::{
        footer::footer,
        head::{HeadProps, head_component},
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
        section class="py-20 bg-slate-950/50" {
                div class="grid grid-cols-1 lg:grid-cols-[1fr_400px] gap-4 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                    main class="flex-1" {
                        "Hola mundo"
                    }
                    aside class="flex-1" {
                        "Una prueba"
                    }
                }
            }
        (footer())
    );

    HttpResponse::Ok().body(markup)
}
