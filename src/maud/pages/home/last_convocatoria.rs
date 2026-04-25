use maud::{Markup, html};

use crate::modules::convocatorias::domain::convocatoria::Convocatoria;

pub struct LastConvocatoriaItem {
    pub title: String,
    pub company: String,
    pub location: String,
    pub url: String,
    pub image: String,
}

impl From<Convocatoria> for LastConvocatoriaItem {
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

pub struct LastConvocatoriaProps {
    pub items: Vec<LastConvocatoriaItem>,
}

pub fn last_convocatoria(props: LastConvocatoriaProps) -> Markup {
    html!(
        section class="py-20 bg-slate-950/50" id="ofertas" {
            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                h2 class="text-3xl font-bold mb-10" {
                    "Convocatorias Recientes"
                }
                div class="grid grid-cols-1 lg:grid-cols-2 gap-6" {
                    @for convocatoria in props.items {
                        div class="bg-theme-glass p-6 rounded-2xl flex flex-col md:flex-row gap-4 items-start hover-glow transition" {
                            img src=(convocatoria.image) class="w-16 h-16 object-contain rounded-xl bg-white shrink-0 flex items-center justify-center border border-white/10" { }
                            div class="flex flex-col grow flex-1 w-full" {
                                div class="flex flex-col md:flex-row justify-between items-start gap-4" {
                                    div class="flex-1" {
                                        h3 class="font-bold text-lg" {
                                            (convocatoria.title)
                                        }
                                        p class="text-blue-400 text-sm font-mediuml" {
                                            (convocatoria.company)
                                        }
                                        span class="bg-blue-500/20 text-blue-300 text-[10px] px-2 py-1 rounded-md font-bold" {
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
                    }
                }

                // div class="mt-10 text-center" {
                //     a href="#" class="border border-rose-700 hover:bg-rose-800 px-8 py-3 rounded-xl transition font-semibold" {
                //         "Ver todas las convocatorias"
                //     }
                // }
            }
        }
    )
}
