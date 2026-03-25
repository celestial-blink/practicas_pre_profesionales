use maud::{Markup, html};

use crate::maud::components::convocatoria_item::{ConvocatoriaItem, convocatoria_item};

pub fn convocatoria_section(children: Markup, title: &str) -> Markup {
    html! {
        section class="flex flex-col gap-2 bg-dark-card h-full p-8 rounded-xl relative" {
            div class="w-full h-64 bg-grid-dark-fade absolute top-0 left-0 opacity-70 z-0" {}
            h2 class="text-3xl font-bold text-white sticky top-0 z-10 py-6" id="ultimas_covocatorias" {
                (title)
            }
            (children)
        }
    }
}
