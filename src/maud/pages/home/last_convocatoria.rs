use maud::{Markup, html};

use crate::{config::DOMAIN, maud::components::convocatoria::convocatoria_min_item::{
    ConvocatoriaMinItem, convocatoria_min_content_item,
}};

pub struct LastConvocatoriaProps {
    pub items: Vec<ConvocatoriaMinItem>,
}

pub fn last_convocatoria(props: LastConvocatoriaProps) -> Markup {
    html!(
        section class="py-20 bg-slate-950/50" id="practicas" {
            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                h2 class="text-3xl font-bold mb-10" {
                    "Convocatorias Recientes"
                }
                div class="grid grid-cols-1 lg:grid-cols-2 gap-6" {
                    @for convocatoria in props.items {
                        (convocatoria_min_content_item(convocatoria))
                    }
                }

                div class="mt-10 text-center" {
                    a href=(format!("{}/busqueda", DOMAIN)) class="border border-rose-700 hover:bg-rose-800 px-8 py-3 rounded-xl transition font-semibold" {
                        "Ver todas las practicas"
                    }
                }
            }
        }
    )
}
