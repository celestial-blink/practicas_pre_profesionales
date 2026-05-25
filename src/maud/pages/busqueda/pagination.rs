use maud::{Markup, html};

use crate::{
    helpers::t_pagination::TPaginationCore,
    modules::ofertas::application::dtos::ofertas_filter_params_dto::OfertasFilterParamsDto,
};

pub struct PaginationProps {
    pub total_pages: u32,
    pub page: u32,
    pub query_params: OfertasFilterParamsDto,
}

pub fn pagination(props: PaginationProps) -> Markup {
    let pages = TPaginationCore::new(props.total_pages, props.page).pages_to_vec();

    html!(
        nav class="pt-8 flex justify-center space-x-2" aria-label="Navegacion de paginas" {
            ul class="flex gap-2 flex-wrap"{
                @if pages.len() > 1 && props.page > 1 {
                    li class="flex items-center justify-center w-10 h-10 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white transition-colors cursor-pointer" {
                        a {
                            svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                                path d="M15 6l-6 6l6 6" { }
                            }
                        }
                    }
                }
                @for page in pages.iter() {
                    @if *page == props.page {
                        li class="flex items-center justify-center w-10 h-10 rounded-full font-semibold text-white bg-white/10 transition-colors cursor-pointer" {
                            span {
                                (page)
                            }
                        }
                    } @else {
                        li class="flex items-center justify-center w-10 h-10 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white transition-colors cursor-pointer" {
                            a href=(format!("?{}", serde_qs::to_string(&props.query_params).unwrap_or("".to_string()))) class="text-white" {
                                (page)
                            }
                        }
                    }
                }
                @if pages.len() > 1 && props.page < props.total_pages {
                    li class="flex items-center justify-center w-10 h-10 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white transition-colors cursor-pointer" {
                        a {
                            svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                path path stroke="none" d="M0 0h24v24H0z" fill="none"  { }
                                path d="M9 6l6 6l-6 6" { }
                            }
                        }
                    }
                }
            }
        }
    )
}
