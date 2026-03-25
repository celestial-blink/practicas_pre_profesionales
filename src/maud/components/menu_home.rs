use maud::{Markup, html, PreEscaped};

#[derive(serde::Deserialize)]
pub struct Departamento {
    id: i32,
    nombre: String,
    alias: String,
}

#[derive(serde::Deserialize)]
pub struct FormacionAcademica {
    id: i32,
    nombre: String,
    alias: String,
}

pub fn menu_home(departamentos: Vec<Departamento>, formacion_academica: Vec<FormacionAcademica>) -> Markup {
    html! {
        p class="text-slate-300 font-bold" { "Buscar por:" }
        nav {
            ul {
                li {
                    details class="group border-b border-slate-800 hover:border-rose-500 transition-colors duration-300" {
                        summary class="list-none cursor-pointer py-4 group-hover:text-rose-500 transition-colors duration-300" {
                            (PreEscaped("&#x276F;".repeat(2))) " Modalidad"
                        }
                        menu class="flex flex-col gap-2" {
                            li {
                                a class="block p-2 rounded-md bg-slate-800 hover:bg-slate-700" {
                                    "Practicas preprofesionales"
                                }
                            }
                            li {
                                a class="block p-2 rounded-md bg-slate-800 hover:bg-slate-700" {
                                    "Practicas profesionales"
                                }
                            }
                        }
                        br;
                    }
                }
                li {
                    details class="group border-b border-slate-800 hover:border-rose-500 transition-colors duration-300" {
                        summary class="list-none cursor-pointer py-4 group-hover:text-rose-500 transition-colors duration-300" {
                            (PreEscaped("&#x276F;".repeat(2))) " Departamento"
                        }
                        menu class="flex flex-col gap-2 max-h-60 overflow-auto" {
                            @for departamento in departamentos {
                                li {
                                    a href="#" class="block p-2 rounded-md bg-slate-800 hover:bg-slate-700" {
                                        (departamento.nombre)
                                    }
                                }
                            }
                        }
                        br;
                    }
                }
                li {
                    details class="group border-b border-slate-800 hover:border-rose-500 transition-colors duration-300" {
                        summary class="list-none cursor-pointer py-4 group-hover:text-rose-500 transition-colors duration-300" {
                            (PreEscaped("&#x276F;".repeat(2))) " Formación académica"
                        }
                        menu class="flex flex-col gap-2 max-h-60 overflow-auto" {
                            @for formacion_academica in formacion_academica {
                                li {
                                    a href="#" class="block p-2 rounded-md bg-slate-800 hover:bg-slate-700" {
                                        (formacion_academica.nombre)
                                    }
                                }
                            }
                        }
                        br;
                    }
                }
            }
        }
    }
}
