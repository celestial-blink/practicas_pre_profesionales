use maud::html;

pub fn top_search() -> maud::Markup {
    html! {
        div class="bg-theme-glass p-4 rounded-2xl mb-8 flex flex-col md:flex-row gap-4" {
            div class="grow relative" {
                svg class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500 w-5 h-5" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                    path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                    path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" { }
                    path d="M21 21l-6 -6" { }
                }
                input type="text" class="w-full bg-slate-950 border border-slate-700/50 rounded-xl py-3 pl-12 pr-4 focus:outline-none focus:ring-2 focus:ring-rose-500 transition" placeholder="Palabras clave (ej. Backend, Finanzas...)";
            }
            div class="md:w-64 relative" {
                svg class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500 w-5 h-5" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                    path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                    path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" { }
                    path d="M17.657 16.657l-4.243 4.243a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 11.314 0" { }
                }
                select class="w-full bg-slate-950 border border-slate-700/50 rounded-xl py-3 pl-12 pr-4 focus:outline-none focus:ring-2 focus:ring-rose-500 appearance-none text-slate-400" {
                    option value="" selected {
                        "Todo el Perú"
                    }
                    option value="lima" {
                        "Lima"
                    }
                    option value="arequipa" {
                        "Arequipa"
                    }
                }
            }
            button type="button" class="bg-rose-600 hover:bg-rose-700 text-white font-bold py-3 px-8 rounded-xl transition" {
                "Buscar"
            }
        }
    }
}
