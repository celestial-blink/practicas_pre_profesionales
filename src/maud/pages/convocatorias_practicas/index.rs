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
            convocatorias_practicas::{convocatoria_item::convocatoria_item_view, meta::meta},
            general::header_items::header_items,
            not_found::component::{NotFoundComponentProps, not_found_component},
        },
    },
    modules::convocatorias::{
        application::{
            dtos::get_all_actives_params_dto::GetAllActivesParamsDto,
            get_all_actives::GetAllActives, get_one_by_alias::GetOneByAlias,
        },
        infrastructure::queries::mariadb_query::MariaDbQuery,
    },
};

#[get("/convocatorias-practicas/{alias}")]
pub async fn convocatorias_practicas(
    state: Data<RwLock<State>>,
    alias: web::Path<String>,
) -> HttpResponse {
    let query_repository = MariaDbQuery::new();
    let get_one_by_alias = GetOneByAlias::new(&query_repository);

    let state = state.read().unwrap();

    let convocatoria = get_one_by_alias
        .execute(&state.db, alias.into_inner())
        .await;

    if convocatoria.is_ok() {
        let params = GetAllActivesParamsDto {
            offset: 0,
            limit: 10,
            include_texto: false,
        };

        let get_all_actives = GetAllActives::new(query_repository);
        let convocatorias = get_all_actives.execute(&state.db, params).await;

        let convocatorias = match convocatorias {
            Ok(convocatorias) => convocatorias
                .into_iter()
                .filter(|conv| conv.id != convocatoria.as_ref().unwrap().id)
                .map(|conv| conv.into())
                .collect(),
            Err(_) => vec![],
        };

        let mut meta = meta();
        let og_title = format!(
            "{} - Practicas Pre y Profesionales Perú",
            convocatoria.as_ref().unwrap().titulo
        );
        meta.insert("og:title".to_owned(), og_title.clone());

        let content = html! {
            (head_component(HeadProps {
                title: og_title,
                metadata: None,
                alternative_metadata: Some(meta),
                canonical: Some(format!("https://practicasperu.com/convocatorias_practicas/{}", convocatoria.as_ref().unwrap().alias)),
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
                        (convocatoria_item_view(convocatoria.unwrap()))
                    }
                    aside class="flex-1" {
                        (last_convocatoria_view(convocatorias))
                    }
                }
            }
            (header(header_items()))
            (footer())
        };

        HttpResponse::Ok().body(content.into_string())
    } else {
        let content = html! {
            (head_component(HeadProps {
                title: "Convocatoria no encontrada".to_string(),
                metadata: None,
                alternative_metadata: None,
                canonical: Some("https://practicasperu.com/convocatorias_practicas".to_string()),
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
                        title: "Convocatoria no encontrada",
                        description: "La convocatoria que buscas no existe o ha sido eliminada.",
                    })
                )
            }
            (header(header_items()))
            (footer())
        };

        HttpResponse::NotFound().body(content.into_string())
    }
}
