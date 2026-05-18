use maud::html;

use crate::data::static_data::DEPARTAMENTOS;
use crate::modules::organizaciones::domain::organizacion::Organizacion;

pub struct TopSearchProps<'t> {
    pub search: Option<String>,
    pub departamento: u8,
    pub organizaciones: &'t Vec<Organizacion>,
}

pub fn top_search(props: TopSearchProps) -> maud::Markup {
    html! {
        div class="bg-theme-glass p-4 rounded-2xl mb-8 flex flex-col lg:flex-row gap-4" {
            div class="grow relative" {
                svg class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500 w-5 h-5" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                    path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                    path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" { }
                    path d="M21 21l-6 -6" { }
                }
                input
                type="text"
                class="w-full bg-slate-950 border border-slate-700/50 rounded-xl py-3 pl-12 pr-4 focus:outline-none focus:ring-2 focus:ring-rose-500 transition"
                name="search"
                id="search"
                value=[props.search]
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
                name="departamento"
                id="departamento"
                onchange="set_departamento('departamento',this.value)" {
                    option value="" selected[props.departamento == 0] { "Todos" }
                    @for departamento in DEPARTAMENTOS.iter() {
                        option value=(departamento.id) selected[props.departamento == departamento.id as u8] {
                            (departamento.nombre)
                        }
                    }
                }
            }
            details class="lg:hidden" {
                summary class="text-slate-300 cursor-pointer" { "Ver mas filtros" }
                br;
                div class="flex flex-col gap-4" {
                    div class="space-y-8" {
                        div {
                            h3 class="text-sm font-bold text-slate-300 uppercase tracking-widest mb-4" {
                                "Organización (Máximo 3)"
                            }
                            div class="space-y-3" data-id="input_search_customized" {
                                input type="hidden" name="organizacion" data-ref="query_params";
                                div class="flex items-center flex-wrap gap-2 cursor-pointer group" data-selected="selected_container" onclick="handle_unset_item(event)" { }
                                input
                                    type="text"
                                    class="w-full bg-slate-900 border border-slate-700/50 rounded-xl p-3 focus:outline-none focus:ring-2 focus:ring-rose-500 transition"
                                    name="search"
                                    placeholder="Buscar organización..."
                                    autocomplete="off"
                                    oninput="handle_search(event)"
                                    onfocus="handle_search_focus(event)";
                                menu class="hidden flex-col border border-slate-700/50 rounded-xl p-3 bg-slate-900" data-menu="search_list" onclick="handle_set_item(event)" {
                                    @for organizacion in props.organizaciones {
                                        li {
                                            button type="button" class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1" {
                                                (organizacion.nombre_comercial)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            h3 class="text-sm font-bold text-slate-300 uppercase tracking-widest mb-4" {
                                "Modalidad de prácticas"
                            }
                            div class="space-y-3" {
                                label class="flex items-center space-x-3 cursor-pointer group" {
                                    input type="checkbox" class="w-4 h-4 rounded border-slate-700";
                                    span class="text-slate-400 group-hover:text-white transition" {
                                        "Pre profesionales"
                                    }
                                }
                                label class="flex items-center space-x-3 cursor-pointer group" {
                                    input type="checkbox" class="w-4 h-4 rounded border-slate-700";
                                    span class="text-slate-400 group-hover:text-white transition" {
                                        "Profesionales"
                                    }
                                }
                            }
                        }
                        div {
                            h3 class="text-sm font-bold text-slate-300 uppercase tracking-widest mb-4" {
                                "Nivel académico"
                            }
                            div class="space-y-3" {
                                label class="flex items-center space-x-3 cursor-pointer group" {
                                    input type="checkbox" class="w-4 h-4 rounded border-slate-700";
                                    span class="text-slate-400 group-hover:text-white transition" {
                                        "Estudiantes técnicos"
                                    }
                                }
                                label class="flex items-center space-x-3 cursor-pointer group" {
                                    input type="checkbox" class="w-4 h-4 rounded border-slate-700";
                                    span class="text-slate-400 group-hover:text-white transition" {
                                        "Egresados técnicos"
                                    }
                                }
                                label class="flex items-center space-x-3 cursor-pointer group" {
                                    input type="checkbox" class="w-4 h-4 rounded border-slate-700";
                                    span class="text-slate-400 group-hover:text-white transition" {
                                        "Estudiantes universitarios"
                                    }
                                }
                                label class="flex items-center space-x-3 cursor-pointer group" {
                                    input type="checkbox" class="w-4 h-4 rounded border-slate-700";
                                    span class="text-slate-400 group-hover:text-white transition" {
                                        "Egresados universitarios"
                                    }
                                }
                                label class="flex items-center space-x-3 cursor-pointer group" {
                                    input type="checkbox" class="w-4 h-4 rounded border-slate-700";
                                    span class="text-slate-400 group-hover:text-white transition" {
                                        "Bachilleres"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            button type="button" class="bg-rose-600 hover:bg-rose-700 text-white font-bold py-3 px-8 rounded-xl transition" {
                "Buscar"
            }
        }
    }
}
