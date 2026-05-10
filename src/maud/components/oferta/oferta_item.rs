use maud::{Markup, html};
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::{
    config::DOMAIN, helpers::t_get_departamento_by_id::get_departamento_by_id,
    modules::ofertas::domain::oferta::Oferta,
};

pub struct OfertaItem {
    pub titulo: String,
    pub alias: String,
    pub nombre_org: String,
    pub alias_org: String,
    pub logo_org: String,
    pub modalidad_practicas: i8,
    pub vacantes: i16,
    pub subvencion: rust_decimal::Decimal,
    pub fecha_fin_oferta: PrimitiveDateTime,
    pub formacion: String,
    pub region: String,
    pub id_region: i8,
    pub distrito: String,
    pub niveles: String,
}

impl From<Oferta> for OfertaItem {
    fn from(oferta: Oferta) -> Self {
        Self {
            titulo: oferta.titulo,
            alias: oferta.alias,
            nombre_org: oferta.nombre_org,
            alias_org: oferta.alias_org,
            logo_org: oferta.logo_org,
            modalidad_practicas: oferta.modalidad_practicas,
            vacantes: oferta.vacantes,
            subvencion: oferta.subvencion,
            fecha_fin_oferta: oferta.fecha_fin_oferta,
            formacion: oferta.formacion,
            region: oferta.region,
            id_region: oferta.id_region,
            distrito: oferta.distrito,
            niveles: oferta.niveles,
        }
    }
}

pub fn oferta_item(oferta: OfertaItem) -> Markup {
    let oferta_url = format!("{}/ofertas-practicas/{}", DOMAIN, oferta.alias);
    let org_url = format!("{}/organizacion/{}", DOMAIN, oferta.alias_org);
    let image_url = format!(
        "{}/public/images/organizaciones/{}",
        DOMAIN, oferta.logo_org
    );

    let target_region = get_departamento_by_id(oferta.id_region as u32);
    let region_url_element = if target_region.is_some() {
        let region = target_region.unwrap();
        let region_url = format!("{}/ofertas-practicas?region={}", DOMAIN, region.id);
        html!(
            a href=(region_url) class="text-blue-400 text-base font-medium hover:underline" target="_blank" {
                (region.nombre)
            }
        )
    } else {
        html!("")
    };

    let now = OffsetDateTime::now_utc();
    let its_expired = oferta.fecha_fin_oferta < PrimitiveDateTime::new(now.date(), now.time());

    html!(
        article class="bg-theme-glass p-6 rounded-2xl flex flex-col md:flex-row gap-4 items-start transition hover:outline-1 outline-offset-2 outline-rose-900" {
            @if its_expired {
                div class="flex gap-1 w-max bg-red-500/20 text-red-500 px-2 py-1 rounded-full font-bold absolute top-0 right-0" {
                    span class="animate-pulse" {
                        "●"
                    }
                    "Convocatoria finalizada"
                }
            } @else {
                div class="flex gap-1 w-max bg-green-500/20 text-green-500 px-2 py-1 rounded-full font-bold absolute top-0 right-0" {
                    span class="animate-pulse" {
                        "●"
                    }
                    "Convocatoria abierta"
                }
            }
            img src=(image_url) class="w-16 h-16 object-contain rounded-xl bg-white shrink-0 flex items-center justify-center border border-white/10" { }
            div class="flex flex-col grow flex-1 w-full" {
                div class="flex flex-col md:flex-row justify-between items-start gap-4" {
                    div class="flex-1 flex flex-col gap-1" {
                        h3 class="font-bold text-lg" {
                            (oferta.titulo)
                        }
                        p {
                            a href=(org_url) class="text-blue-400 text-base font-medium hover:underline" target="_blank" {
                                (oferta.nombre_org)
                            }
                        }
                        span class="bg-blue-500/20 text-blue-300 w-max text-sm px-2 py-1 rounded-md" {
                            (oferta.distrito) " - " (region_url_element)
                        }
                    }
                    div class="h-full self-center w-full md:w-max" {
                        a href=(oferta_url) class="bg-rose-500 block w-full text-center hover:bg-rose-600 transition-colors text-white text-base px-4 py-2 rounded-md font-bold" {
                            "Ver más"
                        }
                    }
                }
            }
        }
    )
}
