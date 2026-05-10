use maud::{Markup, html};

use crate::modules::convocatorias::domain::convocatoria::Convocatoria;

pub struct ConvocatoriaMinItem {
    pub title: String,
    pub company: String,
    pub location: String,
    pub url: String,
    pub image: String,
    pub org_url: String,
}

impl From<Convocatoria> for ConvocatoriaMinItem {
    fn from(convocatoria: Convocatoria) -> Self {
        Self {
            title: convocatoria.titulo,
            company: convocatoria.nombre_org,
            location: convocatoria.departamentos,
            url: format!("/convocatorias-practicas/{}", convocatoria.alias),
            image: format!("/public/images/organizaciones/{}", convocatoria.logo_org),
            org_url: format!("/organizacion/{}", convocatoria.alias_org),
        }
    }
}

pub fn convocatoria_min_content_item(convocatoria: ConvocatoriaMinItem) -> Markup {
    html!(
        article class="bg-theme-glass p-6 rounded-2xl flex flex-col md:flex-row gap-4 items-start transition" {
            img src=(convocatoria.image) class="w-16 h-16 object-contain rounded-xl bg-white shrink-0 flex items-center justify-center border border-white/10" { }
            div class="flex flex-col grow flex-1 w-full" {
                div class="flex flex-col md:flex-row justify-between items-start gap-4" {
                    div class="flex-1 flex flex-col gap-1" {
                        h3 class="font-bold text-lg" {
                            (convocatoria.title)
                        }
                        p {
                            a href=(convocatoria.org_url) class="text-blue-400 text-base font-medium hover:underline" target="_blank" {
                                (convocatoria.company)
                            }
                        }
                        span class="bg-blue-500/20 text-blue-300 w-max text-sm px-2 py-1 rounded-md" {
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
        article class="bg-theme-glass p-6 rounded-2xl flex flex-col gap-4 items-start transition hover:outline-1 outline-offset-2 outline-rose-900" {
            img src=(convocatoria.image) class="w-16 h-16 object-contain rounded-xl bg-white shrink-0 flex items-center justify-center border border-white/10" { }
            div class="flex flex-col grow flex-1 w-full" {
                div class="flex flex-col justify-between items-start gap-4" {
                    div class="flex-1 flex flex-col gap-1" {
                        h3 class="font-bold text-lg text-rose-600" {
                            a href=(&convocatoria.url) class="text-rose-600 hover:text-rose-500 hover:underline" {
                                (convocatoria.title)
                            }
                        }
                        p {
                            a href=(convocatoria.org_url) class="text-blue-400 text-base font-medium hover:underline" target="_blank" {
                                (convocatoria.company)
                            }
                        }
                        span class="bg-blue-500/20 text-blue-200 w-max text-sm px-2 py-1 rounded-md" {
                            (convocatoria.location)
                        }
                    }
                    div class="h-full self-center w-full" {
                        a href=(convocatoria.url) class="bg-rose-50 block w-full text-center hover:bg-rose-100 hover:outline-2 outline-offset-2 outline-rose-600 transition-colors text-rose-600 text-base px-4 py-2 rounded-md font-bold" {
                            "Ver más"
                        }
                    }
                }
            }
        }
    )
}
