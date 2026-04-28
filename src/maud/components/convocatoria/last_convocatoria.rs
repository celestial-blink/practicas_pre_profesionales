use maud::{Markup, html};

use crate::maud::components::convocatoria::convocatoria_min_item::{
    ConvocatoriaMinItem, convocatoria_min_content_sm_item,
};

pub fn last_convocatoria_view(convocatorias: Vec<ConvocatoriaMinItem>) -> Markup {
    if convocatorias.is_empty() {
        return html!();
    }
    html!(
        section {
            div {
                h2 class="text-xl font-bold mb-4" {
                    "Ultimas convocatorias"
                }
                div class="flex flex-col gap-6" {
                    @for convocatoria in convocatorias {
                        (convocatoria_min_content_sm_item(convocatoria))
                    }
                }
            }
        }
    )
}
