use maud::{Markup, PreEscaped, html};
use time;

use crate::modules::convocatorias::domain::convocatoria::Convocatoria;

pub fn convocatoria_item_view(convocatoria: Convocatoria) -> Markup {
    // compara usando microsegundos
    let finished = convocatoria.fin_convocatoria.microsecond() < time::OffsetDateTime::now_utc().microsecond();

    html!(
        section class="bg-theme-glass p-8 rounded-4xl border-blue-500/20" {
            div {
                @if finished {
                    div class="flex gap-1 w-max bg-red-500/20 text-red-500 px-2 py-1 rounded-full font-bold" {
                        // point animate pulse
                        span class="animate-pulse" {
                            "●"
                        }
                        "Convocatoria finalizada"
                    }
                } @else {
                    div class="flex gap-1 w-max bg-green-500/20 text-green-500 px-2 py-1 rounded-full font-bold" {
                        span class="animate-pulse" {
                            "●"
                        }
                        "Convocatoria activa"
                    }
                }
                br;
                h1 class="text-3xl md:text-4xl font-extrabold" {
                    (convocatoria.titulo)
                }
                p class="text-blue-400 font-semibold text-lg mt-1" {
                    (convocatoria.nombre_org)
                }
            }
            br;
            div class="flex flex-col gap-8" {
                div class="flex-1" {
                    h2 class="text-xl font-bold" {
                        "Descripción:"
                    }
                    p class="text-slate-200 text-lg" {
                        "Hay " span class="text-white font-bold" { (convocatoria.vacantes) } " en " span class="text-white font-bold" { (convocatoria.nombre_org) } " publico convocatorias de practicas " span class="text-white font-bold" { (convocatoria.modalidades) } ". para las carreras de: " (convocatoria.carreras)
                    }
                }
                div class="flex-1 grid grid-cols-1 md:grid-cols-3 gap-4" {
                    div class="flex flex-row items-center gap-4 p-4 bg-purple-500/10 rounded-2xl border border-purple-500/5" {
                        (PreEscaped("<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"flex-none text-purple-500\" width=\"48\" height=\"48\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path stroke=\"none\" d=\"M0 0h24v24H0z\" fill=\"none\" /><path d=\"M22 9l-10 -4l-10 4l10 4l10 -4v6\" /><path d=\"M6 10.6v5.4a6 3 0 0 0 12 0v-5.4\" /></svg>"))
                        div class="flex flex-col" {
                            p class="text-sm text-slate-400 font-bold" {
                                "Niveles Académicos"
                            }
                            p class="text-sm font-semibold" {
                                (convocatoria.nivel_estudios)
                            }
                        }
                    }
                    div class="flex flex-row items-center gap-4 p-4 bg-blue-500/10 rounded-2xl border border-blue-500/5" {
                        (PreEscaped("<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"flex-none text-blue-500\" width=\"48\" height=\"48\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path stroke=\"none\" d=\"M0 0h24v24H0z\" fill=\"none\" /><path d=\"M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0\" /><path d=\"M17.657 16.657l-4.243 4.243a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 11.314 0\" /></svg>"))
                        div class="flex flex-col" {
                            p class="text-sm text-slate-400 font-bold" {
                                "Departamentos"
                            }
                            p class="text-sm font-semibold" {
                                (convocatoria.departamentos)
                            }
                        }
                    }
                    div class="flex flex-row items-center gap-4 p-4 bg-green-500/10 rounded-2xl border border-green-500/5" {
                        (PreEscaped("<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"flex-none text-green-500\" width=\"48\" height=\"48\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path stroke=\"none\" d=\"M0 0h24v24H0z\" fill=\"none\" /><path d=\"M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0\" /><path d=\"M3 8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -8\" /><path d=\"M18 12h.01\" /><path d=\"M6 12h.01\" /></svg>"))
                        div class="flex flex-col" {
                            p class="text-sm text-slate-400 font-bold" {
                                "Subvención"
                            }
                            p class="text-sm font-semibold" {
                                (convocatoria.subvenciones)
                            }
                        }
                    }
                }
            }
        }
        br;
        section class="bg-theme-glass p-8 rounded-4xl border-blue-500/20" {
            h2 class="text-xl font-bold" {
                "Lista de vancantes:"
            }
            div class="flex flex-col gap-4 text-slate-200 text-lg" {
                (PreEscaped(convocatoria.texto.unwrap_or_default()))
            }
        }
    )
}
