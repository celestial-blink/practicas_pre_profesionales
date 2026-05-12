use std::sync::RwLock;

use actix_web::{get, web::Data};
use maud::{Markup, html};

use crate::{
    general_types::State,
    helpers::t_get_departamento_by_id::get_departamento_by_id,
    maud::{
        components::{
            footer::footer,
            head::{HeadProps, head_component},
            header::header,
        },
        pages::{departamentos::hero, general::header_items::header_items},
    },
    modules::ofertas::{
        application::get_count_ofertas_by_departamento,
        infrastructure::queries::mariadb_query::MariaDbQuery,
    },
};

struct DepartamentosResult {
    pub vacantes: rust_decimal::Decimal,
    pub departamento: String,
    pub alias: String,
}

#[get("/departamentos")]
pub async fn departamentos_view(state: Data<RwLock<State>>) -> Markup {
    let infrastructure = MariaDbQuery;
    let get_count_ofertas =
        get_count_ofertas_by_departamento::GetCountOfertasByDepartamento::new(infrastructure);

    let state = state.read().unwrap();

    let ofertas = get_count_ofertas.execute(&state.db).await;

    let departamentos: Vec<DepartamentosResult> = match ofertas {
        Ok(ofertas) => ofertas
            .into_iter()
            .map(|oferta| {
                let departamento = get_departamento_by_id(oferta.id_departamento as u32);
                if departamento.is_some() {
                    let departamento = departamento.unwrap();
                    DepartamentosResult {
                        vacantes: oferta.vacantes,
                        departamento: departamento.nombre,
                        alias: departamento.alias,
                    }
                } else {
                    DepartamentosResult {
                        vacantes: rust_decimal::Decimal::from(0),
                        departamento: "".to_owned(),
                        alias: "".to_owned(),
                    }
                }
            })
            .collect(),
        Err(_) => vec![],
    };

    let total_vacantes: rust_decimal::Decimal = departamentos.iter().map(|d| d.vacantes).sum();
    let total_vacantes_lima: rust_decimal::Decimal = departamentos
        .iter()
        .filter(|d| d.alias == "lima")
        .map(|d| d.vacantes)
        .sum();
    let total_vacantes_provincias: rust_decimal::Decimal = departamentos
        .iter()
        .filter(|d| d.alias != "lima")
        .map(|d| d.vacantes)
        .sum();

    html! {
        (head_component(HeadProps {
            title: "Lista de departamentos en el Perú".to_owned(),
            metadata: None,
            canonical: Some("https://www.practicasperupro.com/departamentos".to_owned()),
            scripts_extra: None,
            css_extra: None,
            include_analytics: true,
            include_ads: true,
            text_extra: None,
        }))
        (header(header_items()))
        (hero::hero())
        section class="py-20 bg-slate-950/50" {
            div class="flex flex-col gap-4 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                main class="flex-1" {
                    div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12" {
                        div class="bg-theme-glass p-6 rounded-2xl flex justify-center items-end gap-4" {
                            svg class="bg-rose-500/10 p-3 rounded-xl text-rose-500" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"  {
                                path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                                path d="M4 21v-15c0 -1 1 -2 2 -2h5c1 0 2 1 2 2v15" { }
                                path d="M16 8h2c1 0 2 1 2 2v11" { }
                                path d="M3 21h18" { }
                                path d="M10 12v.01" { }
                                path d="M10 16v.01" { }
                                path d="M10 8v.01" { }
                                path d="M7 12v.01" { }
                                path d="M7 16v.01" { }
                                path d="M7 8v.01" { }
                                path d="M17 12v.01" { }
                                path d="M17 16v.01" { }
                            }
                            p class="text-2xl font-bold" {
                                (total_vacantes) span class="text-sm text-slate-400 tracking-wider" { " Total de vacantes " }
                            }
                        }
                        div class="bg-theme-glass p-6 rounded-2xl flex justify-center items-end gap-4" {
                            svg class="bg-purple-500/10 p-3 rounded-xl text-purple-500" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"  {
                                path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                                path d="M4 21v-15c0 -1 1 -2 2 -2h5c1 0 2 1 2 2v15" { }
                                path d="M16 8h2c1 0 2 1 2 2v11" { }
                                path d="M3 21h18" { }
                                path d="M10 12v.01" { }
                                path d="M10 16v.01" { }
                                path d="M10 8v.01" { }
                                path d="M7 12v.01" { }
                                path d="M7 16v.01" { }
                                path d="M7 8v.01" { }
                                path d="M17 12v.01" { }
                                path d="M17 16v.01" { }
                            }
                            p class="text-2xl font-bold" {
                                (total_vacantes_lima) span class="text-sm text-slate-400 tracking-wider" { " Vacantes en Lima" }
                            }
                        }
                        div class="bg-theme-glass p-6 rounded-2xl flex justify-center items-end gap-4" {
                            svg class="bg-emerald-500/10 p-3 rounded-xl text-emerald-500" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"  {
                                path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                                path d="M3 7l6 -3l6 3l6 -3v13l-6 3l-6 -3l-6 3v-13" { }
                                path d="M9 4v13" { }
                                path d="M15 7v13" { }
                            }
                            p class="text-2xl font-bold" {
                                (total_vacantes_provincias) span class="text-sm text-slate-400 tracking-wider" { " Vacantes en Provincias" }
                            }
                        }
                    }
                    br;
                    br;
                    div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-12" {
                        @for (index, departamento) in departamentos.iter().enumerate() {
                            a href=(format!("/departamento/{}", departamento.alias)) class="bg-theme-glass p-6 rounded-2xl flex flex-col relative hover:-translate-y-1 hover:bg-rose-500/10 hover:outline-2 outline-rose-500/30 outline-offset-2 transition-all duration-300" target="_blank" {
                                @if !departamento.alias.is_empty() {
                                    @if index < 3 {
                                        div class="absolute top-4 right-4 w-3 h-3 bg-purple-500 rounded-full animate-pulse" { }
                                    }
                                    h3 class="font-bold text-lg" {
                                        (departamento.departamento)
                                    }
                                    p class="text-blue-400 text-sm font-semibold" {
                                        (format!("{} vacantes", departamento.vacantes))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        (footer())
    }
}
