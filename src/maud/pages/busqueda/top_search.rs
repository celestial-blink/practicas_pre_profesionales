use maud::html;

use crate::{
    data::static_data::DEPARTAMENTOS,
    modules::ofertas::application::dtos::ofertas_filter_params_dto::OfertasFilterParamsDto,
};

pub struct TopSearchProps<'t> {
    pub query_params: &'t OfertasFilterParamsDto,
}

pub fn top_search(props: TopSearchProps) -> maud::Markup {
    html! {
        form class="bg-theme-glass p-4 rounded-2xl mb-8 flex flex-col lg:flex-row gap-4" onkeydown="handle_prevent_submit_on_key_enter(event)" {
            div class="grow relative" {
                svg class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500 w-5 h-5" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                    path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                    path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" { }
                    path d="M21 21l-6 -6" { }
                }
                input
                    type="search"
                    class="w-full bg-slate-950 border border-slate-700/50 rounded-xl py-3 pl-12 pr-4 focus:outline-none focus:ring-2 focus:ring-rose-500 transition"
                    name="search"
                    id="search"
                    value=[&props.query_params.search]
                    data-ref="query_params"
                    placeholder="Palabras clave (ej. Backend, Finanzas...)";
            }
            div class="lg:w-64 relative" {
                svg class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500 w-5 h-5" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                    path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                    path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" { }
                    path d="M17.657 16.657l-4.243 4.243a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 11.314 0" { }
                }
                select
                    class="w-full bg-slate-950 border border-slate-700/50 rounded-xl py-3 pl-12 pr-4 focus:outline-none focus:ring-2 focus:ring-rose-500 appearance-none text-slate-400"
                    name="id_region"
                    id="id_region"
                    data-ref="query_params"
                    {
                        option value="" selected[props.query_params.id_region.unwrap_or(0) == 0] { "Todos" }
                        @for departamento in DEPARTAMENTOS.iter() {
                            option value=(departamento.id) selected[props.query_params.id_region.unwrap_or(0) == departamento.id as i8] {
                                (departamento.nombre)
                            }
                        }
                    }
            }
            button type="submit" class="bg-rose-600 hover:bg-rose-700 text-white font-bold py-3 px-8 rounded-xl transition" {
                "Buscar"
            }
        }
    }
}
