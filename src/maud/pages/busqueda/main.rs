use maud::{Markup, html};
use rust_decimal::prelude::ToPrimitive;

use crate::{
    maud::{
        components::oferta::oferta_item::oferta_item,
        pages::busqueda::pagination::{PaginationProps, pagination},
    },
    modules::ofertas::{
        application::dtos::ofertas_filter_params_dto::OfertasFilterParamsDto,
        domain::oferta::Oferta,
    },
};

#[derive(Debug)]
pub struct MainProps {
    pub total_ofertas: u32,
    pub ofertas: Vec<Oferta>,
    pub ofertas_vencidas: Vec<Oferta>,
    pub per_page: u32,
    pub query_params: OfertasFilterParamsDto,
    pub limit: u32,
}

pub fn main(props: MainProps) -> Markup {
    let total_pages = (props.total_ofertas as f64 / props.per_page as f64)
        .ceil()
        .to_u32()
        .unwrap_or(0);

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
        (pagination(PaginationProps {
            total_pages,
            query_params: props.query_params,
            limit: props.limit,
        }))
    )
}
