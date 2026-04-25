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
            convocatorias_practicas::convocatoria_item::convocatoria_item_view,
            general::header_items::header_items,
        },
    },
    modules::convocatorias::{
        application::get_one_by_alias::GetOneByAlias,
        infrastructure::queries::mariadb_query::MariaDbQuery,
    },
};

#[get("/convocatorias-practicas/{alias}")]
pub async fn convocatorias_practicas(state: Data<State>, alias: web::Path<String>) -> HttpResponse {
    let query_repository = MariaDbQuery;
    let get_one_by_alias = GetOneByAlias::new(query_repository);
    let convocatoria = get_one_by_alias
        .execute(&state.db, alias.into_inner())
        .await;

    if convocatoria.is_ok() {
        let content = html! {
            (head_component(HeadProps {
                title: convocatoria.as_ref().unwrap().titulo.clone(),
                metadata: None,
                canonical: Some("https://practicasperu.com/convocatorias_practicas".to_string()),
                scripts_extra: None,
                css_extra: None,
                include_analytics: true,
                include_ads: true,
            }))
            br;
            br;
            section class="py-20 bg-slate-950/50" {
                div class="grid grid-cols-1 lg:grid-cols-[1fr_400px] gap-4 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                    main class="flex-1" {
                        (convocatoria_item_view(convocatoria.unwrap()))
                    }
                    aside class="flex-1" {
                        "Aside"
                    }
                }
            }
            br;
            br;
            (header(header_items()))
            (footer())
        };

        HttpResponse::Ok().body(content.into_string())
    } else {
        let content = html! {
            (head_component(HeadProps {
                title: "Convocatoria no encontrada".to_string(),
                metadata: None,
                canonical: Some("https://practicasperu.com/convocatorias_practicas".to_string()),
                scripts_extra: None,
                css_extra: None,
                include_analytics: true,
                include_ads: true,
            }))

            section class="py-20 bg-slate-950/50" {
                div class="grid grid-cols-1 lg:grid-cols-[1fr_400px] gap-4 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                    main class="flex flex-col gap-4" {
                        h1 class="text-2xl font-bold text-white mb-4" {
                            "Convocatoria no encontrada"
                        }
                        p class="text-white" {
                            "La convocatoria que buscas no existe o ha sido eliminada."
                        }
                        a class="text-blue-500 hover:underline mt-4 inline-block" href="/convocatorias-practicas" {
                            "Volver a la lista de convocatorias"
                        }
                    }
                    aside class="flex flex-col gap-4" {
                        "Aside"
                    }
                }
            }

            (header(header_items()))
            (footer())
        };

        HttpResponse::NotFound().body(content.into_string())
    }
}
