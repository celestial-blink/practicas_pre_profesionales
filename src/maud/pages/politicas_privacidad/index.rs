use actix_web::get;
use maud::{Markup, html};

use crate::maud::pages::general::header_items::header_items;
use crate::maud::{
    components::{
        footer::footer,
        head::{HeadProps, head_component},
        header::header,
    },
    pages::politicas_privacidad::meta::meta,
};

#[get("/politicas-privacidad")]
pub async fn politicas_privacidad() -> Markup {
    html! {
        (head_component(HeadProps {
            title: "Politicas de Privacidad - Prácticas Perú Pro".to_owned(),
            metadata: Some(meta()),
            alternative_metadata: None,
            canonical: Some("https://www.practicasperupro.com/politicas-privacidad".to_owned()),
            scripts_extra: None,
            css_extra: None,
            include_analytics: true,
            include_ads: true,
            text_extra: None,
        }))
        (header(header_items()))
        br;
        br;
        section class="py-20 bg-slate-950/50 min-h-screen text-slate-300" {
            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                main class="bg-theme-glass p-8 md:p-12 rounded-3xl border border-slate-800/50 shadow-2xl backdrop-blur-md" {
                    h1 class="text-3xl md:text-4xl font-extrabold text-transparent bg-clip-text bg-linear-to-r from-rose-400 via-pink-500 to-purple-500 mb-2" {
                        "Política de Privacidad"
                    }
                    p class="text-sm text-slate-400 mb-8" {
                        "Última actualización: 31 de mayo de 2026"
                    }

                    p class="leading-relaxed text-slate-300 mb-8" {
                        "En Prácticas Perú Pro (en adelante, \"la Plataforma\") respetamos tu privacidad. Esta Política de Privacidad describe cómo tratamos la información cuando visitas nuestro sitio web, en cumplimiento de la Ley N° 29733, Ley de Protección de Datos Personales del Perú y su Reglamento aprobado mediante D.S. N° 003-2013-JUS."
                    }

                    div class="flex flex-col gap-8" {
                        // Section 1
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "1" }
                                "Responsable del sitio web"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "El responsable de la Plataforma es:"
                            }
                            div class="overflow-x-auto mt-2" {
                                table class="min-w-full divide-y divide-slate-800 text-sm text-left" {
                                    tbody class="divide-y divide-slate-850" {
                                        tr {
                                            td class="py-3 px-4 font-semibold text-white w-1/3" { "Nombre / Razón social" }
                                            td class="py-3 px-4 text-slate-300" { "Prácticas Perú Pro" }
                                        }
                                        tr {
                                            td class="py-3 px-4 font-semibold text-white" { "Sitio web" }
                                            td class="py-3 px-4 text-slate-300" { "https://www.practicasperupro.com" }
                                        }
                                        tr {
                                            td class="py-3 px-4 font-semibold text-white" { "Correo de contacto" }
                                            td class="py-3 px-4 text-slate-300" {
                                                a href="mailto:contacto@practicasperupro.com" class="text-rose-400 hover:text-rose-300 transition-colors" { "contacto@practicasperupro.com" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Section 2
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "2" }
                                "Datos que no recopilamos"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "La Plataforma es un sitio exclusivamente informativo. No recopilamos, almacenamos ni procesamos datos personales de nuestros visitantes. Esto significa que:"
                            }
                            ul class="list-disc pl-6 flex flex-col gap-2 text-slate-300" {
                                li { "No existen formularios de registro ni cuentas de usuario." }
                                li { "No solicitamos nombre, correo electrónico, teléfono ni ningún otro dato personal para acceder al contenido." }
                                li { "No compartimos información de usuarios con terceros, porque no disponemos de ella." }
                                li { "No realizamos perfilamiento ni segmentación de usuarios." }
                            }
                        }

                        // Section 3
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "3" }
                                "Datos de navegación y cookies"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Como la mayoría de sitios web, nuestra Plataforma puede registrar de forma automática ciertos datos técnicos de navegación a través de herramientas de análisis (por ejemplo, Google Analytics). Estos datos incluyen:"
                            }
                            ul class="list-disc pl-6 flex flex-col gap-2 text-slate-300" {
                                li { "Dirección IP anonimizada." }
                                li { "Tipo de navegador y sistema operativo." }
                                li { "Páginas visitadas y tiempo de permanencia." }
                                li { "Fuente de tráfico (buscador, acceso directo, redes sociales)." }
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Esta información es agregada, anónima y se usa únicamente para mejorar el funcionamiento del sitio. No permite identificar a ningún usuario de forma individual."
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Puedes desactivar las cookies desde la configuración de tu navegador. Ten en cuenta que esto podría afectar algunas funcionalidades de navegación."
                            }
                        }

                        // Section 4
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "4" }
                                "Derechos ARCO"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Conforme a la Ley N° 29733, toda persona tiene derecho a ejercer los derechos ARCO sobre sus datos personales:"
                            }
                            div class="grid grid-cols-1 sm:grid-cols-2 gap-4 my-2" {
                                div class="p-4 rounded-xl bg-slate-900/50 border border-slate-800/50" {
                                    h3 class="font-bold text-white mb-1" { "Acceso" }
                                    p class="text-sm text-slate-400" { "Conocer qué datos tuyos tenemos" }
                                }
                                div class="p-4 rounded-xl bg-slate-900/50 border border-slate-800/50" {
                                    h3 class="font-bold text-white mb-1" { "Rectificación" }
                                    p class="text-sm text-slate-400" { "Corregir datos inexactos" }
                                }
                                div class="p-4 rounded-xl bg-slate-900/50 border border-slate-800/50" {
                                    h3 class="font-bold text-white mb-1" { "Cancelación" }
                                    p class="text-sm text-slate-400" { "Solicitar la eliminación de tus datos" }
                                }
                                div class="p-4 rounded-xl bg-slate-900/50 border border-slate-800/50" {
                                    h3 class="font-bold text-white mb-1" { "Oposición" }
                                    p class="text-sm text-slate-400" { "Oponerte al tratamiento de tus datos" }
                                }
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Dado que la Plataforma no recopila datos personales, el ejercicio de estos derechos no aplica en la práctica. No obstante, si tuvieras alguna consulta al respecto, puedes escribirnos a "
                                a href="mailto:contacto@practicasperupro.com" class="text-rose-400 hover:text-rose-300 transition-colors" { "contacto@practicasperupro.com" }
                                "."
                            }
                        }

                        // Section 5
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "5" }
                                "Menores de edad"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "La Plataforma está dirigida a estudiantes y egresados en búsqueda de prácticas, por lo que puede ser visitada por personas menores de 18 años. Al no recopilar datos personales, no existe riesgo de tratamiento de datos de menores. No obstante, recomendamos que los menores de edad cuenten con la supervisión de sus padres o tutores al navegar por internet."
                            }
                        }

                        // Section 6
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "6" }
                                "Seguridad del sitio"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Adoptamos medidas técnicas razonables para garantizar la seguridad e integridad del sitio web, incluyendo el uso de protocolo HTTPS para el cifrado de las comunicaciones entre el usuario y el servidor. Aunque ningún sistema es infalible, trabajamos para mantener la Plataforma protegida frente a accesos no autorizados."
                            }
                        }

                        // Section 7
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "7" }
                                "Cambios en esta política"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Nos reservamos el derecho de actualizar esta Política de Privacidad en cualquier momento, especialmente si en el futuro incorporamos funcionalidades que impliquen el tratamiento de datos personales. Cualquier cambio será publicado en esta misma página con la fecha de actualización correspondiente. Te recomendamos revisarla periódicamente."
                            }
                        }

                        // Section 8
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "8" }
                                "Autoridad de protección de datos"
                            }
                            p class="leading-relaxed text-slate-300 font-semibold" {
                                "Si consideras que tus derechos en materia de protección de datos han sido vulnerados, puedes presentar una reclamación ante la autoridad competente en el Perú:"
                            }
                            div class="p-4 rounded-xl bg-slate-900/50 border border-slate-800/50 flex flex-col gap-1 w-fit" {
                                h3 class="font-bold text-white text-sm md:text-base" { "Autoridad Nacional de Protección de Datos Personales (ANPD)" }
                                p class="text-xs md:text-sm text-slate-400" { "Dependiente del Ministerio de Justicia y Derechos Humanos del Perú" }
                                a href="https://www.gob.pe/minjus" target="_blank" rel="noopener noreferrer" class="text-xs text-rose-400 hover:text-rose-350 transition-colors w-fit mt-1" { "www.gob.pe/minjus" }
                            }
                        }

                        // Section 9
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "9" }
                                "Contacto"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Para cualquier consulta relacionada con esta Política de Privacidad, puedes contactarnos en: "
                                a href="mailto:contacto@practicasperupro.com" class="text-rose-400 hover:text-rose-300 transition-colors" { "contacto@practicasperupro.com" }
                            }
                        }
                    }
                }
            }
        }
        (footer())
    }
}
