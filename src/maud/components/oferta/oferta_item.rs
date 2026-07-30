use maud::{Markup, html};
use rust_decimal::prelude::ToPrimitive;
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::{
    config::DOMAIN, helpers::t_get_departamento::get_departamento_by_id,
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
            id_region: oferta.id_region,
            distrito: oferta.distrito,
            niveles: oferta.niveles,
        }
    }
}

pub fn oferta_item(oferta: OfertaItem) -> Markup {
    let oferta_url = format!("{}/oferta-practicas/{}", DOMAIN, oferta.alias);
    let org_url = format!("{}/organizacion/{}", DOMAIN, oferta.alias_org);
    let image_url = format!(
        "{}/public/images/organizaciones/{}",
        DOMAIN, oferta.logo_org
    );

    let target_region = get_departamento_by_id(oferta.id_region as u32);
    let region_url_element = if target_region.is_some() {
        let region = target_region.unwrap();
        let region_url = format!("{}/oferta-practicas?region={}", DOMAIN, region.id);
        html!(
            a href=(region_url) class="text-blue-400 text-base underline" target="_blank" {
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
                div class="flex gap-1 w-max bg-red-500/20 text-red-500 px-2 py-1 rounded-full text-xs z-10 absolute top-4 right-4" {
                    span class="animate-pulse" {
                        "●"
                    }
                    "Expirado"
                }
            } @else {
                div class="flex gap-1 w-max bg-green-500/20 text-green-500 px-2 py-1 rounded-full text-xs z-10 absolute top-4 right-4" {
                    span class="animate-pulse" {
                        "●"
                    }
                    "Vigente"
                }
            }
            img src=(image_url) class="w-16 h-16 object-contain rounded-xl bg-white shrink-0 flex items-center justify-center border border-white/10" { }
            div class="flex flex-col grow flex-1 w-full" {
                div class="flex flex-col md:flex-row justify-between items-start gap-4" {
                    div class="flex-1 flex flex-col gap-1" {
                        p {
                            a href=(org_url) class="text-blue-400 text-base font-medium hover:underline" target="_blank" {
                                (oferta.nombre_org)
                            }
                        }
                        h3 class="font-bold text-base text-slate-200 hover:text-rose-500 hover:underline" {
                            a href=(oferta_url) target="_blank" {
                                (oferta.titulo)
                            }
                        }
                        p class="text-slate-300" {
                            span class="font-bold text-white" { "Formación: " }
                            (oferta.formacion)
                        }
                        p class="text-slate-300" {
                            span class="font-bold text-white" { "Para: " }
                            (oferta.niveles)
                        }
                        br;
                        div class="flex flex-row flex-wrap gap-2" {
                            @if oferta.subvencion.to_i32().unwrap_or(0) > 0 {
                                span class="bg-green-500/20 text-green-300 w-max text-sm px-2 py-1 rounded-md" {
                                    "S/. " (oferta.subvencion)
                                }
                            }
                            span class="bg-purple-500/20 text-purple-300 w-max text-sm px-2 py-1 rounded-md" {
                                (oferta.vacantes) " vacantes"
                            }
                            @if oferta.modalidad_practicas == 0 {
                                span class="bg-rose-500/20 text-rose-300 w-max text-sm px-2 py-1 rounded-md" {
                                    "Pre profesional"
                                }
                            } @else if oferta.modalidad_practicas == 1 {
                                span class="bg-rose-500/20 text-rose-300 w-max text-sm px-2 py-1 rounded-md" {
                                    "Profesional"
                                }
                            } @else if oferta.modalidad_practicas == 2 {
                                span class="bg-rose-500/20 text-rose-300 w-max text-sm px-2 py-1 rounded-md" {
                                    "Pre y profesional"
                                }
                            }
                            span class="bg-blue-500/20 text-blue-300 w-max text-sm px-2 py-1 rounded-md" {
                                (oferta.distrito) ", " (region_url_element)
                            }
                        }
                    }
                    div class="h-full w-full md:w-max md:flex-[0.3] self-end" {
                        a href=(oferta_url) class="bg-rose-500 block w-full text-center hover:bg-rose-600 transition-colors text-white text-base px-4 py-2 rounded-md font-bold" {
                            "Ver más"
                        }
                    }
                }
            }
        }
    )
}
