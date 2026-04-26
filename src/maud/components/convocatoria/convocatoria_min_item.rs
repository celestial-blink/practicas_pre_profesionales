use maud::{Markup, html};

use crate::modules::convocatorias::domain::convocatoria::Convocatoria;

pub struct ConvocatoriaMinItem {
    pub title: String,
    pub company: String,
    pub location: String,
    pub url: String,
    pub image: String,
}

impl From<Convocatoria> for ConvocatoriaMinItem {
    fn from(convocatoria: Convocatoria) -> Self {
        Self {
            title: convocatoria.titulo,
            company: convocatoria.nombre_org,
            location: convocatoria.departamentos,
            url: format!("/convocatorias-practicas/{}", convocatoria.alias),
            image: format!("/public/images/organizaciones/{}", convocatoria.logo_org),
        }
    }
}

pub fn convocatoria_min_content_item(convocatoria: ConvocatoriaMinItem) -> Markup {
    html!(
        article class="bg-theme-glass p-6 rounded-2xl flex flex-col md:flex-row gap-4 items-start hover-glow transition" {
            img src=(convocatoria.image) class="w-16 h-16 object-contain rounded-xl bg-white shrink-0 flex items-center justify-center border border-white/10" { }
            div class="flex flex-col grow flex-1 w-full" {
                div class="flex flex-col md:flex-row justify-between items-start gap-4" {
                    div class="flex-1 flex flex-col gap-1" {
                        h3 class="font-bold text-lg" {
                            (convocatoria.title)
                        }
                        p class="text-blue-400 text-base font-mediuml" {
                            (convocatoria.company)
                        }
                        span class="bg-blue-500/20 text-blue-300 w-max text-base px-2 py-1 rounded-md font-bold" {
                            (convocatoria.location)
                        }
                    }
                    div class="h-full self-center w-full md:w-max" {
                        a href=(convocatoria.url) class="bg-rose-500 block w-full text-center hover:bg-rose-600 transition-colors text-white text-base px-4 py-2 rounded-md font-bold" {
                            "Ver más"
                        }
                    }
                }
            }
        }
    )
}

pub fn convocatoria_min_content_sm_item(convocatoria: ConvocatoriaMinItem) -> Markup {
    html!(
        article class="bg-theme-glass p-6 rounded-2xl flex flex-col gap-4 items-start hover-glow transition" {
            img src=(convocatoria.image) class="w-16 h-16 object-contain rounded-xl bg-white shrink-0 flex items-center justify-center border border-white/10" { }
            div class="flex flex-col grow flex-1 w-full" {
                div class="flex flex-col justify-between items-start gap-4" {
                    div class="flex-1 flex flex-col gap-1" {
                        h3 class="font-bold text-lg" {
                            (convocatoria.title)
                        }
                        p class="text-blue-400 text-base font-mediuml" {
                            (convocatoria.company)
                        }
                        span class="bg-blue-500/20 text-blue-300 w-max text-base px-2 py-1 rounded-md font-bold" {
                            (convocatoria.location)
                        }
                    }
                    div class="h-full self-center w-full" {
                        a href=(convocatoria.url) class="bg-rose-500 block w-full text-center hover:bg-rose-600 transition-colors text-white text-base px-4 py-2 rounded-md font-bold" {
                            "Ver más"
                        }
                    }
                }
            }
        }
    )
}
