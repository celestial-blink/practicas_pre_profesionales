use maud::{Markup, html};

pub fn aside_filters() -> Markup {
    html!(
        div class="flex flex-col gap-4" {
            div class="space-y-8" {
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
                div {
                    h3 class="text-sm font-bold text-slate-300 uppercase tracking-widest mb-4" {
                        "Organización"
                    }
                    div class="space-y-3" {
                        label class="flex items-center space-x-3 cursor-pointer group" {
                            input type="checkbox" class="w-4 h-4 rounded border-slate-700";
                            span class="text-slate-400 group-hover:text-white transition" {
                                "SUNARP"
                            }
                        }
                    }
                }
            }
        }
    )
}
