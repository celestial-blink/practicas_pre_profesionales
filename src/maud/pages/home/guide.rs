use maud::{Markup, html};

pub fn guide() -> Markup {
    html!(
        section class="py-24 relative" {
            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center" {
                h2 class="text-3xl font-bold mb-16 italic" {
                    "Consigue tu práctica en 3 pasos"
                }
                div class="grid grid-cols-1 md:grid-cols-3 gap-12" {
                    div class="relative" {
                        div class="w-16 h-16 bg-rose-600 rounded-2xl flex items-center justify-center text-2xl font-bold mx-auto mb-6 shadow-lg shadow-blue-900/50" {
                            "1"
                        }
                        h3 class="font-bold text-lg mb-2" {
                            "Encuentra la oferta de practica " a href="/ofertas" class="text-blue-500 hover:underline" {
                                "aqui"
                            }
                        }
                        p class="text-slate-400 text-sm" {
                            "Busca prácticas y revisa bien los requisitos y fechas antes de postular."
                        }
                    }
                    div class="relative" {
                        div class="w-16 h-16 bg-rose-600 rounded-2xl flex items-center justify-center text-2xl font-bold mx-auto mb-6 shadow-lg shadow-blue-900/50" {
                            "2"
                        }
                        h3 class="font-bold text-lg mb-2" {
                            "Postula correctamente"
                        }
                        p class="text-slate-400 text-sm" {
                            "Lee las bases de la oferta de práctica y envía tu CV según las indicaciones: correo, web o presencial"
                        }
                    }
                    div class="relative" {
                        div class="w-16 h-16 bg-rose-600 rounded-2xl flex items-center justify-center text-2xl font-bold mx-auto mb-6 shadow-lg shadow-blue-900/50" {
                            "3"
                        }
                        h3 class="font-bold text-lg mb-2" {
                            "Espera resultados"
                        }
                        p class="text-slate-400 text-sm" {
                            "Revisa tu correo y prepárate para entrevistas o evaluaciones, también lo indica las bases."
                        }
                    }
                }
            }
        }
    )
}
