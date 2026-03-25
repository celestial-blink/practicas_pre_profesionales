use maud::{html, Markup};

pub fn footer() -> Markup {
    html! {
        footer class="bg-dark-card rounded-xl flex flex-col gap-4 p-8" {
            ul class="flex flex-wrap gap-2 justify-center" {
                li {
                    a href="#" class="text-rose-400 block p-2 rounded-md hover:bg-slate-800" {
                        "Politicas de Privacidad"
                    }
                }
                li {
                    a href="#" class="text-rose-400 block p-2 rounded-md hover:bg-slate-800" {
                        "Terminos y Condiciones"
                    }
                }
                li {
                    a href="#" class="text-rose-400 block p-2 rounded-md hover:bg-slate-800" {
                        "Contacto"
                    }
                }
            }
            p class="text-slate-50 text-center text-sm" {
                "© 2026 Practicas Perú Pro. Todos los derechos reservados."
            }
        }
    }
}
