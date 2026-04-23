use maud::{Markup, html};

pub fn hero() -> Markup {
    html! {
        section  class="relative pt-32 pb-20 overflow-hidden" {
            div class="absolute -top-24 -left-24 w-96 h-96 bg-blue-600/10 rounded-full blur-3xl" { }
            div class="absolute top-1/2 -right-24 w-80 h-80 bg-emerald-600/10 rounded-full blur-3xl" { }

            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative" {
                div class="text-center max-w-3xl mx-auto mb-12" {
                    span class="inline-block py-1 px-3 rounded-full bg-blue-500/10 text-blue-400 text-xs font-semibold uppercase tracking-wider mb-4 border border-blue-500/20" {
                        "Tu primera oportunidad profesional"
                    }

                    h1 class="text-4xl md:text-6xl font-extrabold mb-6 leading-tight" {
                        "Encuentra convocatorias de " span class="text-rose-500" { "Prácticas pre y profesionales" } " en el Perú"
                    }

                    p class="text-lg text-slate-400 mb-10 leading-relaxed" {
                        "Conectamos el talento universitario con las empresas más importantes del país. Filtros inteligentes, y 100% gratuito para estudiantes, egresado y bachilleres."
                    }
                }
            }
        }
    }
}
