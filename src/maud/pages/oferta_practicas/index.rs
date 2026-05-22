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
            convocatoria::last_convocatoria::last_convocatoria_view,
            footer::footer,
            head::{HeadProps, head_component},
            header::header,
        },
        pages::{
            general::header_items::header_items,
            oferta_practicas::oferta_item_view::oferta_item_view,
        },
    },
    modules::{
        convocatorias::{
            application::{
                dtos::get_all_actives_params_dto::GetAllActivesParamsDto,
                get_all_actives::GetAllActives,
            },
            infrastructure::queries::mariadb_query::MariaDbQuery,
        },
        ofertas::{
            application::get_one_by_alias::GetOneByAlias,
            infrastructure::queries::mariadb_query::MariaDbQuery as MariaDbQueryOfertas,
        },
    },
};

#[get("/oferta-practicas/{alias}")]
pub async fn oferta_practicas(
    state: Data<RwLock<State>>,
    alias: web::Path<String>,
) -> HttpResponse {
    let mariadb_oferta_query = MariaDbQueryOfertas {};
    let get_one_by_alias = GetOneByAlias::new(&mariadb_oferta_query);

    let state = state.read().unwrap();

    let oferta = get_one_by_alias
        .execute(&state.db, alias.into_inner())
        .await;

    let mariadb_convocatoria_query = MariaDbQuery;
    let get_last_convocatoria = GetAllActives::new(mariadb_convocatoria_query);
    let last_convocatorias = get_last_convocatoria
        .execute(
            &state.db,
            GetAllActivesParamsDto {
                include_texto: false,
                limit: 10,
                offset: 0,
            },
        )
        .await;

    let last_convocatorias = match last_convocatorias {
        Ok(convocatorias) => convocatorias.into_iter().map(|conv| conv.into()).collect(),
        Err(_) => vec![],
    };

    match oferta {
        Some(oferta) => {
            let html = html! {
                (head_component(HeadProps {
                    title: oferta.titulo.clone(),
                    metadata: None,
                    canonical: Some(format!("https://practicasperu.com/oferta_practicas/{}", oferta.alias.clone())),
                    scripts_extra: None,
                    css_extra: None,
                    include_analytics: true,
                    include_ads: true,
                    text_extra: None,
                }))
                br;
                br;
                section class="py-20 bg-slate-950/50" {
                    div class="grid grid-cols-1 lg:grid-cols-[1fr_400px] gap-4 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                        main class="flex-1" {
                            (oferta_item_view(oferta))
                        }
                        aside class="flex-1" {
                            (last_convocatoria_view(last_convocatorias))
                        }
                    }
                }
                (header(header_items()))
                (footer())
            };
            HttpResponse::Ok().body(html.into_string())
        }
        _ => HttpResponse::NotFound().body("Oferta no encontrada"),
    }
}
