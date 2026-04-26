use maud::{Markup, PreEscaped, html};
use time::{self, OffsetDateTime, PrimitiveDateTime};

use crate::modules::ofertas::domain::oferta::Oferta;

pub fn oferta_item_view(oferta: Oferta) -> Markup {
    // compara usando microsegundos
    let now = OffsetDateTime::now_utc();
    let expired = PrimitiveDateTime::new(now.date(), now.time()) > oferta.fecha_fin_oferta;

    let wrapper_class = "[&_ul]:list-disc [&_ul,&_ol]:ml-4 [&_ol]:list-decimal [&_a]:text-blue-500 [&_a]:hover:underline [&_strong]:text-white";

    html!(
        section class="bg-theme-glass p-8 rounded-4xl border-blue-500/20" {
            div {
                @if expired {
                    div class="flex gap-1 w-max bg-red-500/20 text-red-500 px-2 py-1 rounded-full font-bold" {
                        span class="animate-pulse" {
                            "●"
                        }
                        "Finalizada"
                    }
                } @else {
                    div class="flex gap-1 w-max bg-green-500/20 text-green-500 px-2 py-1 rounded-full font-bold" {
                        span class="animate-pulse" {
                            "●"
                        }
                        "Abierta"
                    }
                }
                br;
                div class="flex gap-2 items-center" {
                    img src=(format!("/public/images/organizaciones/{}", oferta.logo_org)) class="size-20 bg-white p-2 rounded-lg object-contain" alt=(format!("Logo de {}", oferta.nombre_org));

                    div {
                        h1 class="text-3xl md:text-4xl font-extrabold" {
                            (oferta.titulo)
                        }
                        p class="text-blue-400 font-semibold text-lg mt-1" {
                            (oferta.nombre_org)
                        }
                    }
                }
            }
            br;
            div class="flex flex-col gap-8" {
                div class="flex-1 grid grid-cols-1 md:grid-cols-3 gap-4" {
                    div class="flex flex-row items-center gap-4 p-4 bg-purple-500/10 rounded-2xl border border-purple-500/5" {
                        (PreEscaped("<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"flex-none text-purple-500\" width=\"48\" height=\"48\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path stroke=\"none\" d=\"M0 0h24v24H0z\" fill=\"none\" /><path d=\"M22 9l-10 -4l-10 4l10 4l10 -4v6\" /><path d=\"M6 10.6v5.4a6 3 0 0 0 12 0v-5.4\" /></svg>"))
                        div class="flex flex-col" {
                            p class="text-sm text-slate-400 font-bold" {
                                "Modalidad de practicas"
                            }
                            p class="text-sm font-semibold" {
                                @match oferta.modalidad_practicas {
                                    0 => "Pre profesional",
                                    1 => "Profesional",
                                    2 =>  "Pre y profesional",
                                    _ =>  ""
                                }
                            }
                        }
                    }
                    div class="flex flex-row items-center gap-4 p-4 bg-purple-500/10 rounded-2xl border border-purple-500/5" {
                        (PreEscaped("<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"flex-none text-purple-500\" width=\"48\" height=\"48\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path stroke=\"none\" d=\"M0 0h24v24H0z\" fill=\"none\" /><path d=\"M22 9l-10 -4l-10 4l10 4l10 -4v6\" /><path d=\"M6 10.6v5.4a6 3 0 0 0 12 0v-5.4\" /></svg>"))
                        div class="flex flex-col" {
                            p class="text-sm text-slate-400 font-bold" {
                                "Niveles Académicos"
                            }
                            p class="text-sm font-semibold" {
                                (oferta.niveles)
                            }
                        }
                    }
                    div class="flex flex-row items-center gap-4 p-4 bg-blue-500/10 rounded-2xl border border-blue-500/5" {
                        (PreEscaped("<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"flex-none text-blue-500\" width=\"48\" height=\"48\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path stroke=\"none\" d=\"M0 0h24v24H0z\" fill=\"none\" /><path d=\"M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0\" /><path d=\"M17.657 16.657l-4.243 4.243a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 11.314 0\" /></svg>"))
                        div class="flex flex-col" {
                            p class="text-sm text-slate-400 font-bold" {
                                "Lugar"
                            }
                            p class="text-sm font-semibold" {
                                (oferta.distrito) ", " (oferta.region)
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
                                "S/."(oferta.subvencion)
                            }
                        }
                    }
                }
            }
        }
        br;
        section class="bg-theme-glass p-8 rounded-4xl border-blue-500/20" {
            div class=(format!("flex flex-col gap-4 {}", wrapper_class)) {
                div class="flex flex-col gap-2" {
                    h2 class="text-xl font-bold" {
                        "Formación académica:"
                    }
                    p class="text-base text-slate-300 font-semibold" {
                        (oferta.formacion)
                    }
                }
                {
                    @if let Some(funciones) = oferta.funciones {
                        div class="flex flex-col gap-2" {
                            h2 class="text-xl font-bold" {
                                "Funciones:"
                            }
                            div class="text-base text-slate-300 font-semibold" {
                                (PreEscaped(funciones))
                            }
                        }
                    }
                }
                @if let Some(lugar_practicas) = oferta.lugar_practicas {
                    div class="flex flex-col gap-2" {
                        h2 class="text-xl font-bold" {
                            "Lugar de practicas:"
                        }
                        div class="text-base text-slate-300 font-semibold" {
                            (PreEscaped(lugar_practicas))
                        }
                    }
                }
                @if let Some(como_postular) = oferta.como_postular {
                    div class="flex flex-col gap-2" {
                        h2 class="text-xl font-bold" {
                            "Como postular:"
                        }
                        div class="text-base text-slate-300 font-semibold" {
                            (PreEscaped(como_postular))
                        }
                    }
                }
            }
        }
        @if let Some(extra_info) = oferta.extra_info {
            br;
            section class="bg-theme-glass p-8 rounded-4xl border-blue-500/20 relative overflow-hidden before:block bofore:w-full before:h-full before:bg-yellow-500/10 before:absolute before:top-0 before:left-0 before:right-0 before:bottom-0 before:z-0" {
                div class=(format!("flex flex-col gap-4 z-10 relative {}", wrapper_class)) {
                    div class="flex flex-col gap-2" {
                        h2 class="text-xl font-bold" {
                            "Información adicional:"
                        }
                        div class="text-base text-slate-300 font-semibold" {
                            (PreEscaped(extra_info))
                        }
                    }
                }
            }
        }
        @if let Some(bases) = oferta.bases {
            br;
            section class="bg-theme-glass p-8 rounded-4xl border-blue-500/20 relative overflow-hidden before:block bofore:w-full before:h-full before:bg-green-500/10 before:absolute before:top-0 before:left-0 before:right-0 before:bottom-0 before:z-0" {
                div class=(format!("flex flex-col gap-4 z-10 relative {}", wrapper_class)) {
                    div class="flex flex-col gap-2" {
                        h2 class="text-xl font-bold" {
                            "Bases:"
                        }
                        div class="text-base text-slate-300 font-semibold" {
                            (PreEscaped(bases))
                        }
                    }
                }
            }
        }
        br;
        section {
            h2 class="text-xl font-bold" {
                "Lista de vacantes:"
            }
            br;

        }
    )
}
