use maud::{Markup, html};

use crate::{
    maud::components::oferta::oferta_item::oferta_item, modules::ofertas::domain::oferta::Oferta,
};

pub struct MainProps {
    pub total_ofertas: u32,
    pub ofertas: Vec<Oferta>,
    pub ofertas_vencidas: Vec<Oferta>,
}

pub fn main(props: MainProps) -> Markup {
    html!(
        section class="flex flex-col gap-6" {
            @if props.total_ofertas == 0 {
                p class="text-slate-400 text-sm" {
                    "No se encontraron ofertas de practicas"
                }
            } @else {
                p class="text-slate-400 text-sm" {
                    "Mostrando " span class="text-white font-bold" { (props.total_ofertas) } " oportunidades de practicas encontradas"
                }
            }
            @for oferta in props.ofertas {
                (oferta_item(oferta.into()))
            }
            @for oferta in props.ofertas_vencidas {
                (oferta_item(oferta.into()))
            }
        }
    )
}
