use maud::{Markup, html};

pub fn aside_filters() -> Markup {
    html!(
        div class="flex-col gap-4 hidden lg:flex" {
            div class="space-y-8" {
                div {
                    h3 class="text-sm font-bold text-slate-300 uppercase tracking-widest mb-4" {
                        "Organización (Máximo 3)"
                    }
                    div class="space-y-3" id="search_org_container" {
                        input type="hidden" name="organizacion" data-ref="query_params";
                        div class="flex items-center flex-wrap gap-2 cursor-pointer group" id="org_selected_container" onclick="handle_unset_org_item(event)" { }
                        input
                            type="text"
                            class="w-full bg-slate-900 border border-slate-700/50 rounded-xl p-3 focus:outline-none focus:ring-2 focus:ring-rose-500 transition"
                            name="search"
                            placeholder="Buscar organización..."
                            autocomplete="off"
                            onfocus="handle_search_org_focus(event)";

                        menu class="hidden flex-col border border-slate-700/50 rounded-xl p-3 bg-slate-900" id="search_org_list" {
                            li {
                                button type="button" class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1" onclick="handle_set_org_item(event)" {
                                    "Organizacion 1"
                                }
                            }
                            li {
                                button type="button" class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1" onclick="handle_set_org_item(event)" {
                                    "Organizacion 2"
                                }
                            }
                            li {
                                button type="button" class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1" onclick="handle_set_org_item(event)" {
                                    "Organizacion 3"
                                }
                            }
                            li {
                                button type="button" class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1" onclick="handle_set_org_item(event)" {
                                    "Organizacion 4"
                                }
                            }
                            li {
                                button type="button" class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1" onclick="handle_set_org_item(event)" {
                                    "Organizacion 5"
                                }
                            }
                            li {
                                button type="button" class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1" onclick="handle_set_org_item(event)" {
                                    "Organizacion 5"
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
    )
}
