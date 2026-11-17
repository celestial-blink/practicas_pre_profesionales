use maud::html;

pub struct HeroProps {
    pub search: Option<String>,
    pub total: u32,
    pub description: Option<String>,
}

pub fn hero(props: HeroProps) -> maud::Markup {
    let HeroProps {
        search,
        description,
        total,
    } = props;
    html! {
        section class="relative pt-32 pb-20 overflow-hidden bg-slate-950" {
            // Background glowing blur elements in Rose color
            div class="absolute -top-24 -left-24 w-96 h-96 bg-rose-600/10 rounded-full blur-3xl pointer-events-none" { }
            div class="absolute top-1/2 -right-24 w-80 h-80 bg-rose-900/5 rounded-full blur-3xl pointer-events-none" { }

            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10" {
                div class="text-center max-w-3xl mx-auto" {
                    h1 class="text-4xl md:text-7xl font-extrabold mb-6 leading-tight tracking-tight text-white" {
                        @if let Some(q) = search {
                            "Hay " (total) " resultados para \"" (q) "\""
                        } @else {
                            "Busca Prácticas pre y profesionales en Peru"
                        }
                    }

                    @if let Some(desc) = description {
                        p class="text-lg md:text-xl text-slate-400 leading-relaxed max-w-2xl mx-auto" {
                            (desc)
                        }
                    }
                }
            }
        }
    }
}
