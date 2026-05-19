use maud::{Markup, html};

pub struct PaginationProps {
    pub total_pages: u32,
    pub page: u32,
}

pub fn pagination(props: PaginationProps) -> Markup {
    html!(
        nav class="pt-8 flex justify-center space-x-2" aria-label="Navegacion de paginas" {
            ul {
                @for page in 1..=props.total_pages {
                    li {
                        a {
                            (page)
                        }
                    }
                }
            }
        }
    )
}
