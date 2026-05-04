use maud::{Markup, html};
use time::PrimitiveDateTime;

use crate::helpers::t_date_es::{TDate, format_date_human_es};

#[derive(Clone)]
pub struct ConvocatoriaItem {
    pub titulo: String,
    pub alias: String,
    pub alias_org: String,
    pub nombre_org: String,
    pub logo_org: String,
    pub fin_convocatoria: PrimitiveDateTime,
    pub formacion: String,
    pub departamentos: String,
}

pub fn convocatoria_item(prop: ConvocatoriaItem, key: usize) -> Markup {
    let url = format!("/oferta-practicas/{}", prop.alias);

    html! {
        article class="bg-theme-glass rounded-4xl border-blue-500/20 p-8 grid grid-cols-1 md:grid-cols-[64px_1fr] gap-4 z-0" id={(format!("convocatoria_{}", key))} {
            div class="col-span-1" {
                img src=(format!("/public/images/organizaciones/{}", prop.logo_org)) alt=(prop.nombre_org) class="size-16 p-1 rounded-lg bg-white";
            }
            div class="col-span-1 md:col-span-2 md:col-start-2 flex flex-col" {
                p class="text-base font-bold" {
                    a href=(format!("/organizacion/{}", prop.alias_org)) class="text-blue-400 hover:text-blue-500 hover:underline" target="_blank" {
                        (prop.nombre_org)
                    }
                }
                h3 class="text-lg font-bold  my-1" {
                    a href=(&url) class="text-rose-500 hover:underline" target="_blank" {
                        (prop.titulo)
                    }
                }
                p class="text-slate-300" {
                    span class="font-bold text-white" { "Formación: " } (prop.formacion)
                }
                p class="text-slate-300" {
                    span class="font-bold text-white" { "Lugar: " } (prop.departamentos)
                    }
                    p class="text-slate-300" {
                        span class="font-bold text-white" { "Fecha de finalización: " } (format_date_human_es(&TDate::PrimitiveDateTime(prop.fin_convocatoria)))
                    }
            }
            div class="col-span-1 md:col-span-2 md:col-start-2" {
                a href=(url) class="block text-white font-bold bg-rose-800 hover:bg-rose-700 px-8 py-2 rounded-full w-max" target="_blank" {
                    "Ver completo"
                }
            }
        }
    }
}
