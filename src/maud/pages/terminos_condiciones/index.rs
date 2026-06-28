use actix_web::get;
use maud::{Markup, html};

use crate::maud::components::{
    footer::footer,
    head::{HeadProps, head_component},
    header::header,
};
use crate::maud::pages::general::header_items::header_items;
use crate::maud::pages::terminos_condiciones::meta::meta;

#[get("/terminos-condiciones")]
pub async fn terminos_condiciones() -> Markup {
    html! {
        (head_component(HeadProps {
            title: "Términos y Condiciones - Prácticas Perú Pro".to_owned(),
            metadata: Some(meta()),
            alternative_metadata: None,
            canonical: Some("https://www.practicasperupro.com/terminos-condiciones".to_owned()),
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
                        "Términos y Condiciones de Uso"
                    }
                    p class="text-sm text-slate-400 mb-8" {
                        "Última actualización: 31 de mayo de 2026"
                    }

                    div class="flex flex-col gap-8" {
                        // Section 1
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "1" }
                                "Objeto y naturaleza del servicio"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Este sitio web (en adelante, \"la Plataforma\") es un portal informativo que recopila y publica convocatorias de prácticas pre-profesionales y profesionales disponibles en el territorio peruano. La Plataforma actúa exclusivamente como intermediario de información y no garantiza la vigencia, exactitud o disponibilidad de las convocatorias publicadas."
                            }
                            p class="leading-relaxed text-slate-300" {
                                "El acceso y uso de la Plataforma es gratuito y está dirigido a estudiantes, egresados e interesados en oportunidades laborales de prácticas en el Perú."
                            }
                        }

                        // Section 2
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "2" }
                                "Aceptación de los términos"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "El acceso y navegación en la Plataforma implica la aceptación plena y sin reservas de los presentes Términos y Condiciones. Si el usuario no estuviese de acuerdo con alguna de las disposiciones aquí contenidas, deberá abstenerse de utilizar el sitio."
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Nos reservamos el derecho de modificar estos términos en cualquier momento. Las modificaciones entrarán en vigencia desde el momento de su publicación en la Plataforma, por lo que recomendamos revisarlos periódicamente."
                            }
                        }

                        // Section 3
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "3" }
                                "Responsabilidad sobre la información publicada"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "La información sobre convocatorias mostrada en la Plataforma proviene de fuentes públicas o ha sido remitida por terceros (empresas, instituciones u organismos). En consecuencia:"
                            }
                            ul class="list-disc pl-6 flex flex-col gap-2 text-slate-300" {
                                li { "No nos responsabilizamos por la veracidad, vigencia o exactitud de las convocatorias publicadas." }
                                li { "No somos parte en los procesos de selección ni en las relaciones laborales que se deriven de las convocatorias." }
                                li { "No garantizamos que el usuario sea seleccionado en alguna convocatoria mostrada en la Plataforma." }
                                li { "Recomendamos al usuario verificar directamente con la empresa o institución convocante la vigencia de cada oferta." }
                            }
                        }

                        // Section 4
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "4" }
                                "Uso aceptable de la plataforma"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "El usuario se compromete a utilizar la Plataforma de manera responsable y conforme a la normativa vigente peruana. Queda expresamente prohibido:"
                            }
                            ul class="list-disc pl-6 flex flex-col gap-2 text-slate-300" {
                                li { "Reproducir, distribuir o comercializar el contenido de la Plataforma sin autorización previa." }
                                li { "Utilizar herramientas automatizadas (bots, scrapers) para extraer información masiva del sitio." }
                                li { "Realizar acciones que afecten la disponibilidad, seguridad o funcionamiento del sitio." }
                                li { "Publicar o difundir información falsa, engañosa o que infrinja derechos de terceros." }
                            }
                        }

                        // Section 5
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "5" }
                                "Protección de datos personales"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "La Plataforma no recopila ni almacena datos personales de sus usuarios. No se utilizan formularios de registro, cuentas de usuario ni se solicita información personal para acceder al contenido."
                            }
                            p class="leading-relaxed text-slate-300" {
                                "En caso de que en el futuro se incorporen funcionalidades que impliquen el tratamiento de datos personales, se publicará una Política de Privacidad conforme a la Ley N° 29733, Ley de Protección de Datos Personales, y su Reglamento aprobado por D.S. N° 003-2013-JUS."
                            }
                        }

                        // Section 6
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "6" }
                                "Cookies y tecnologías de seguimiento"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "La Plataforma puede utilizar cookies técnicas o de análisis de tráfico (como Google Analytics u otras herramientas similares) con la finalidad exclusiva de mejorar la experiencia del usuario y medir el rendimiento del sitio. Estas cookies no identifican personalmente al usuario."
                            }
                            p class="leading-relaxed text-slate-300" {
                                "El usuario puede desactivar el uso de cookies desde la configuración de su navegador, aunque esto podría afectar algunas funcionalidades del sitio."
                            }
                        }

                        // Section 7
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "7" }
                                "Propiedad intelectual"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "El diseño, estructura, textos, logotipos y demás elementos propios de la Plataforma son titularidad del operador del sitio y están protegidos por la Ley N° 822, Ley sobre el Derecho de Autor. Queda prohibida su reproducción total o parcial sin autorización expresa y por escrito."
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Las marcas, logotipos e información de las empresas e instituciones que figuran en las convocatorias publicadas son propiedad exclusiva de sus respectivos titulares. Su aparición en la Plataforma no implica cesión ni licencia alguna sobre dichos derechos."
                            }
                        }

                        // Section 8
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "8" }
                                "Enlaces a terceros"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "La Plataforma puede contener enlaces a sitios web de terceros (empresas, instituciones, portales de empleo). Dichos enlaces se facilitan únicamente como referencia y no implican ningún respaldo, afiliación ni responsabilidad sobre el contenido o las prácticas de esos sitios. El usuario accede a ellos bajo su propia responsabilidad."
                            }
                        }

                        // Section 9
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "9" }
                                "Limitación de responsabilidad"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "En la máxima medida permitida por la ley peruana, el operador de la Plataforma no será responsable por daños directos, indirectos, incidentales o consecuentes derivados del uso o la imposibilidad de uso del sitio, incluyendo la pérdida de oportunidades laborales, errores en la información de convocatorias, o interrupciones del servicio."
                            }
                        }

                        // Section 10
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "10" }
                                "Marco legal aplicable y resolución de controversias"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Los presentes Términos y Condiciones se rigen por las leyes de la República del Perú. Para cualquier controversia derivada del uso de la Plataforma, las partes se someten a la jurisdicción de los juzgados y tribunales competentes de la ciudad de Lima, renunciando expresamente a cualquier otro fuero que pudiera corresponderles."
                            }
                            p class="leading-relaxed text-slate-300 font-semibold" {
                                "Normativa de referencia aplicable: Código Civil (D. Leg. N° 295), Ley de Protección al Consumidor (Ley N° 29571), Ley de Comercio Electrónico (Ley N° 27291) y Ley de Delitos Informáticos (Ley N° 30096)."
                            }
                        }

                        // Section 11
                        div class="flex flex-col gap-3" {
                            h2 class="text-xl font-bold text-white flex items-center gap-3 border-b border-slate-800 pb-2" {
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-rose-500/10 text-rose-400 text-sm font-bold shrink-0" { "11" }
                                "Contacto"
                            }
                            p class="leading-relaxed text-slate-300" {
                                "Para consultas, reportes de contenido incorrecto o cualquier comunicación relacionada con estos términos, el usuario puede contactarnos a través de: "
                                a href="mailto:contacto@practicasperupro.com" class="text-rose-400 hover:text-rose-300 transition-colors" { "contacto@practicasperupro.com" }
                                " o mediante el formulario de contacto disponible en la Plataforma."
                            }
                        }
                    }
                }
            }
        }
        (footer())
    }
}
