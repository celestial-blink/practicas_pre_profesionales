use maud::{Markup, html};

use crate::modules::organizaciones::domain::organizacion::Organizacion;

pub struct AsideFiltersProps<'t> {
    pub organizaciones: &'t Vec<Organizacion>,
}

pub fn aside_filters<'t>(props: AsideFiltersProps<'t>) -> Markup {
    html!(
        div class="hidden lg:flex flex-col gap-4" {
            form class="space-y-8" onkeydown="handle_prevent_submit_on_key_enter(event)" {
                div {
                    h3 class="text-sm font-bold text-slate-300 uppercase tracking-widest mb-4" {
                        "Organización (Máximo 3)"
                    }
                    div class="space-y-3" data-id="input_search_customized" {
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
                                    button
                                    type="button"
                                    class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1"
                                    data-id=(organizacion.id)
                                    {
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
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Pre profesionales"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" data-ref="query_params";
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
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="1" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Estudiantes técnicos"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="2" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Egresados técnicos"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="3" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Estudiantes universitarios"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="4" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Egresados universitarios"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="5" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Bachilleres"
                            }
                        }
                    }
                }
                button type="button" class="bg-rose-600 hover:bg-rose-700 text-white font-bold py-3 px-8 rounded-xl transition w-full" {
                    "Aplicar filtros"
                }
            }
        }
        dialog class="md:hidden! bg-slate-900 backdrop:bg-black/50 h-max max-h-4/5 w-full max-w-full m-auto mb-0" id="aside_filters_mobile" onclick="handle_close_header_dialog(event, 'aside_filters_mobile')" data-evref="handle_close_header_dialog" {
            form class="flex justify-end p-4 md:p-6 sticky top-0 bg-slate-900" method="dialog" {
                button type="submit" class="text-white" onclick="handle_open_dialog('aside_filters_mobile', false)" {
                    svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M18 6l-12 12" { }
                        path d="M6 6l12 12" { }
                    }

                }
            }
            form class="space-y-8 p-4 md:p-6" onkeydown="handle_prevent_submit_on_key_enter(event)" {
                div {
                    h3 class="text-sm font-bold text-slate-300 uppercase tracking-widest mb-4" {
                        "Organización (Máximo 3)"
                    }
                    div class="space-y-3" data-id="input_search_customized" {
                        div class="flex items-center flex-wrap gap-2 cursor-pointer group" data-selected="selected_container" onclick="handle_unset_item(event)" { }
                        input
                            type="text"
                            class="w-full bg-slate-950 border border-slate-700/50 rounded-xl p-3 focus:outline-none focus:ring-2 focus:ring-rose-500 transition"
                            name="search"
                            placeholder="Buscar organización..."
                            autocomplete="off"
                            oninput="handle_search(event)"
                            onfocus="handle_search_focus(event)";
                        menu class="hidden flex-col border border-slate-700/50 rounded-xl p-3 bg-slate-950" data-menu="search_list" onclick="handle_set_item(event)" {
                            @for organizacion in props.organizaciones {
                                li {
                                    button
                                    type="button"
                                    class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1"
                                    data-id=(organizacion.id)
                                    {
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
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Pre profesionales"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" data-ref="query_params";
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
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="1" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Estudiantes técnicos"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="2" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Egresados técnicos"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="3" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Estudiantes universitarios"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="4" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Egresados universitarios"
                            }
                        }
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700" name="nivel_academico" value="5" data-ref="query_params";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "Bachilleres"
                            }
                        }
                    }
                }
                button type="button" class="bg-rose-600 hover:bg-rose-700 text-white font-bold py-3 px-8 rounded-xl transition w-full" {
                    "Aplicar filtros"
                }
            }
        }
        div class="lg:hidden fixed bottom-4 right-4 z-50 px-2" onclick="handle_open_dialog('aside_filters_mobile', true)" {
            button type="button" class="flex items-center gap-2 bg-sky-600 hover:bg-sky-700 text-white font-bold py-3 px-8 rounded-xl transition" {
                svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"  {
                    path stroke="none" d="M0 0h24v24H0z" fill="none"  { }
                    path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" { }
                    path d="M6 4v4" { }
                    path d="M6 12v8" { }
                    path d="M10 16a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" { }
                    path d="M12 4v10" { }
                    path d="M12 18v2" { }
                    path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0"  { }
                    path d="M18 4v1" { }
                    path d="M18 9v11" { }
                }
                "Filtros"
            }
        }
    )
}
