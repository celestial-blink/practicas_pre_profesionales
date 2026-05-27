use maud::{Markup, html};

use crate::{
    helpers::t_pagination::TPaginationCore,
    modules::ofertas::application::dtos::ofertas_filter_params_dto::OfertasFilterParamsDto,
};

pub struct PaginationProps {
    pub total_pages: u32,
    pub query_params: OfertasFilterParamsDto,
    pub limit: u32,
}

pub fn pagination(props: PaginationProps) -> Markup {
    let page = props.query_params.offset / props.limit as i32 + 1;
    let pages = TPaginationCore::new(props.total_pages, page as u32).pages_to_vec();

    let set_page_params = |page: i32| {
        let mut query = props.query_params.clone();
        query.offset = (page - 1) * props.limit as i32;
        query
    };

    html!(
        nav class="pt-8 flex justify-center space-x-2" aria-label="Navegacion de paginas" {
            ul class="flex gap-2 flex-wrap"{
                @if pages.len() > 1 && page > 1 {
                    li {
                        a
                        href=(format!("?{}", serde_qs::to_string(&set_page_params(page - 1)).unwrap_or("".to_string())))
                        class="flex items-center justify-center w-10 h-10 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white transition-colors cursor-pointer"
                        {
                            svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                                path d="M15 6l-6 6l6 6" { }
                            }
                        }
                    }
                }
                @for p in pages.iter() {
                    @if *p == page as u32 {
                        li class="flex items-center justify-center w-10 h-10 rounded-full font-semibold text-white bg-rose-500" {
                            span {
                                (p)
                            }
                        }
                    } @else {
                        li {
                            a
                            href=(format!("?{}", serde_qs::to_string(&set_page_params(*p as i32)).unwrap_or("".to_string())))
                            class="flex items-center justify-center w-10 h-10 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white transition-colors cursor-pointer" {
                                (p)
                            }
                        }
                    }
                }
                @if pages.len() > 1 && page < props.total_pages as i32 {
                    li {
                        a
                        href=(format!("?{}", serde_qs::to_string(&set_page_params(page as i32 + 1)).unwrap_or("".to_string())))
                        class="flex items-center justify-center w-10 h-10 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white transition-colors cursor-pointer"
                        {
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
