use crate::modules::{
    ofertas::application::dtos::ofertas_filter_params_dto::OfertasFilterParamsDto,
    organizaciones::domain::organizacion::Organizacion,
};

pub struct ResultFiltersProps<'t> {
    pub organizaciones: &'t Vec<Organizacion>,
    pub query_params: &'t OfertasFilterParamsDto,
}

pub fn result_filters(props: ResultFiltersProps) -> maud::Markup {
    if props.query_params.id_organizacion.is_none()
        && props.query_params.modalidad_practicas.is_none()
        && props.query_params.niveles.is_none()
    {
        return maud::html! {};
    }

    let org_selected: Vec<Organizacion> = props
        .organizaciones
        .iter()
        .filter(|&org| {
            props
                .query_params
                .id_organizacion
                .clone()
                .unwrap_or(vec![])
                .contains(&org.id)
        })
        .cloned()
        .collect();

    maud::html! {
        section class="flex flex-col gap-2" {
            h3 class="text-sm font-bold text-slate-300 uppercase tracking-widest" {
                "Filtros aplicados:"
            }
            ul class="flex gap-2 flex-wrap" onclick="handle_remove_filter(event)" {
                @for org in &org_selected {
                    li class="inline-flex gap-2 items-center px-4 py-1 border border-rose-700 rounded-full bg-rose-950/30" {
                        span class="text-sm text-slate-300" {
                            (&org.nombre_comercial[..std::cmp::min(org.nombre_comercial.len(), 10)])
                        }
                        button
                            type="button"
                            class="text-slate-300 hover:text-white transition-colors"
                            data-key="id_organizacion"
                            data-id=(org.id)
                            title="Quitar filtro"
                            aria-label="Quitar filtro"
                            {
                                svg xmlns="http://www.w3.org/2000/svg" class="pointer-events-none" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                    path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                                    path d="M18 6l-12 12"  { }
                                    path d="M6 6l12 12" { }
                                }
                            }
                    }
                }
                @if let Some(modalidad_practicas) = &props.query_params.modalidad_practicas {
                    li class="inline-flex gap-2 items-center px-4 py-1 border border-rose-700 rounded-full bg-rose-950/30" {
                        span class="text-sm text-slate-300" {
                            @match modalidad_practicas {
                                0 => "Pre-profesionales",
                                1 => "Profesionales",
                                2 => "Pre y profesionales",
                                _ => "",
                            }
                        }
                        button
                            type="button"
                            class="text-slate-300 hover:text-white transition-colors"
                            title="Quitar filtro"
                            data-key="modalidad_practicas"
                            data-id=(modalidad_practicas)
                            aria-label="Quitar filtro"
                            {
                                svg xmlns="http://www.w3.org/2000/svg" class="pointer-events-none" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                    path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                                    path d="M18 6l-12 12"  { }
                                    path d="M6 6l12 12" { }
                                }
                            }
                    }
                }
            }
        }
    }
}
