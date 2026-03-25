use maud::{Markup, PreEscaped, html};
use time::{OffsetDateTime, macros::format_description};

pub struct ConvocatoriaItem {
    pub id: i32,
    pub titulo: String,
    pub id_organizacion: i32,
    pub nombre_org: String,
    pub logo_org: String,
    pub fin_convocatoria: OffsetDateTime,
    pub carreras: String,
    pub departamentos: String,
    pub texto: String,
    pub finalizan_todos: bool,
}

pub fn convocatoria_item(prop: ConvocatoriaItem, key: usize) -> Markup {
    html! {
        article class="grid grid-cols-1 md:grid-cols-[64px_1fr] gap-4 z-0" id={(format!("convocatoria_{}", key))} {
            div class="col-span-1" {
                img src=(prop.logo_org) alt=(prop.nombre_org) class="size-16 p-1 rounded-lg bg-white";
            }
            div class="col-span-1 md:col-span-2 md:col-start-2 flex flex-col" {
                @if prop.fin_convocatoria > OffsetDateTime::now_utc() {
                    span class="bg-green-900 text-green-200 text-xs px-2 rounded-full w-max mb-2"{
                        (PreEscaped("&#8226;"))" Abierta"
                    }
                } @else {
                    span class="bg-red-900 text-red-200 text-xs px-2 rounded-full w-max mb-2"{
                        (PreEscaped("&#8226;"))" Cerrada"
                    }
                }
                p class="text-base font-bold text-slate-300" {
                    (prop.nombre_org)
                }
                h3 class="text-2xl font-bold text-white my-1" {
                    (prop.titulo)
                }
                p class="text-slate-300" {
                    span class="font-bold" { "Carreras requeridas: " } (prop.carreras)
                }
                p class="text-slate-300" {
                    span class="font-bold" { "Departamentos: " } (prop.departamentos)
                    }
                    p class="text-slate-300" {
                        span class="font-bold" { "Fecha de cierre: " } (prop.fin_convocatoria.format(format_description!("[day]-[month]-[year]")).unwrap_or_default())
                    }
            }
            div class="col-span-1 md:col-span-2 md:col-start-2" {
                a href="#" class="block text-white font-bold bg-rose-800 hover:bg-rose-700 px-8 py-2 rounded-full w-max" {
                    "Ver convocatoria"
                }
            }
        }
    }
}
