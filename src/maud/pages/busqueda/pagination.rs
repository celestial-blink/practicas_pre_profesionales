use maud::{Markup, html};

use crate::helpers::t_pagination::TPaginationCore;

pub struct PaginationProps {
    pub total_pages: u32,
    pub page: u32,
}

pub fn pagination(props: PaginationProps) -> Markup {
    let pages = TPaginationCore::new(props.total_pages, props.page).pages_to_vec();

    html!(
        nav class="pt-8 flex justify-center space-x-2" aria-label="Navegacion de paginas" {
            ul class="flex gap-2 flex-wrap"{

                @for page in pages {
                    li class="flex items-center justify-center w-10 h-10 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white transition-colors cursor-pointer" {
                        a {
                            (page)
                        }
                    }
                }

            }
        }
    )
}
