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
            },
            departamento::{
                hero::{HeroProps, hero},
                meta::meta,
                top_search::{TopSearchProps, top_search},
            },
            general::header_items::header_items,
            not_found::component::{NotFoundComponentProps, not_found_component},
        },
    },
    modules::{
        ofertas::application::{
            dtos::ofertas_filter_params_dto::OfertasFilterParamsDto, ofertas_filter::OfertasFilter,
        },
        organizaciones::{
            application::get_one_by_alias::{self, GetOneByAlias},
            infrastructure::mariadb_query_repository::MariadbQueryRepository,
        },
    },
    types::departamento::Departamento,
};

#[get("/departamento/{alias}")]
pub async fn departamento_view(
    state: Data<RwLock<State>>,
    query: serde_qs::actix::QsQuery<OfertasFilterParamsDto>,
    alias: web::Path<String>,
) -> HttpResponse {
    let state = state.read().unwrap();

    let query_org_infrastructure = MariadbQueryRepository;
    let get_org_by_alias = GetOneByAlias::new(&query_org_infrastructure);
    let target_departamento = get_org_by_alias.execute(&state.db.clone(), alias.clone());
    // todo

    if target_departamento.is_none() {
        let content = html! {
            (head_component(HeadProps {
                title: "Organizacion no encontrada".to_string(),
                metadata: None,
                alternative_metadata: None,
                canonical: Some(format!("https://www.practicasperu.com/organizacion/{}", alias.into_inner())),
                scripts_extra: None,
                css_extra: None,
                include_analytics: true,
                include_ads: true,
                text_extra: None,
            }))
            br;
            br;
            main {
                (
                    not_found_component(NotFoundComponentProps {
                        title: "Departamento no encontrado",
                        description: "El departamento que estás buscando no existe o ha sido eliminado.",
                    })
                )
            }
            (header(header_items()))
            (footer())
        };
        return HttpResponse::NotFound()
            .content_type("text/html")
            .body(content.into_string());
    }

    let Departamento { id, nombre, .. } = target_departamento.unwrap();

    let mut query_clone = query.clone();
    query_clone.id_region = Some(id as i8);

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

    let mut meta = meta();
    meta.insert(
        "og:title".to_owned(),
        format!(
            "Practicas pre profesionales en {} - Practicas Pre y Profesionales en Perú",
            nombre
        ),
    );

    let markup = html!(
            (head_component(HeadProps {
            title: format!("Practicas pre profesionales en {} - Practicas Pre y Profesionales en Perú", nombre),
            metadata: None,
            alternative_metadata: Some(meta),
            canonical: Some(format!("https://www.practicasperupro.com/departamento/{}", alias)),
            scripts_extra: Some(vec!["/public/js/pages/busqueda.js".to_owned()]),
            css_extra: None,
            include_analytics: true,
            include_ads: true,
            text_extra: Some(vec![
                format!("<script>const organizaciones = {};</script>", serde_json::to_string(&state.cache.organizaciones).unwrap())
            ])
        }))
        (header(header_items()))
        (hero(HeroProps {
            title: format!("Prácticas pre y profesionales en {} - Practicas Pre y Profesionales en Perú", nombre),
            description: format!("Encuentra las mejores prácticas pre y profesionales en {}", nombre),
        }))
        section class="py-20 bg-slate-950/50" {
            div class="flex flex-col gap-4 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
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
