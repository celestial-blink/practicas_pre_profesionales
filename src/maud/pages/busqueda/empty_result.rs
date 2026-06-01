use maud::html;

pub fn empty_result() -> maud::Markup {
    html! {
        div class="grow flex flex-col items-center justify-center text-center p-8 py-16" {
            div class="flex flex-col items-center justify-center relative mb-6" {
                div class="absolute inset-0 bg-indigo-500/10 rounded-full blur-xl scale-125" { }
                div class="relative w-16 h-16 bg-linear-to-b from-slate-800 to-slate-900 border border-slate-700/50 rounded-2xl flex items-center justify-center text-slate-400 shadow-lg" {
                    svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M5.039 5.062a7 7 0 0 0 9.91 9.89m1.584 -2.434a7 7 0 0 0 -9.038 -9.057" { }
                        path d="M3 3l18 18" { }
                    }
                }
                h3 class="text-lg font-bold text-slate-200 mb-2" {
                    "No encontramos coincidencias"
                }
                p class="text-slate-400 max-w-lg mb-6" {
                    "Intenta cambiar los términos de búsqueda, remover filtros activos o verificar que no haya errores ortográficos."
                }
                p class="text-slate-400 max-w-lg mb-6" {
                    "Pero puedes revisar convocatorias pasadas para que vayas preparando tu postulación en futuras oportunidades. 👇"
                }
            }
        }
    }
}
