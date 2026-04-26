use maud::{Markup, html};

use crate::maud::components::convocatoria::convocatoria_min_item::{
    ConvocatoriaMinItem, convocatoria_min_content_sm_item,
};

pub struct LastConvocatoriaProps {
    pub items: Vec<ConvocatoriaMinItem>,
}

pub fn last_convocatoria_view(props: LastConvocatoriaProps) -> Markup {
    html!(
        section {
            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                h2 class="text-xl font-bold mb-4" {
                    "Ultimas convocatorias"
                }
                div class="flex flex-col gap-6" {
                    @for convocatoria in props.items {
                        (convocatoria_min_content_sm_item(convocatoria))
                    }
                }
            }
        }
    )
}
