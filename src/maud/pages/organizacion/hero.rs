use maud::html;

pub struct HeroProps {
    pub title: String,
    pub description: String,
}

pub fn hero(props: HeroProps) -> maud::Markup {
    html! {
        section class="relative pt-32 pb-20 overflow-hidden bg-slate-950" {
            // Background glowing blur elements in Rose color
            div class="absolute -top-24 -left-24 w-96 h-96 bg-rose-600/10 rounded-full blur-3xl pointer-events-none" { }
            div class="absolute top-1/2 -right-24 w-80 h-80 bg-rose-900/5 rounded-full blur-3xl pointer-events-none" { }

            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10" {
                div class="text-center max-w-3xl mx-auto" {
                    // Accent badge
                    span class="inline-flex items-center gap-1.5 py-1 px-3 rounded-full bg-rose-500/10 text-rose-400 text-xs font-semibold uppercase tracking-wider mb-6 border border-rose-500/20" {
                        span class="w-1.5 h-1.5 rounded-full bg-rose-500 animate-pulse" {}
                        "Convocatorias por organizacion"
                    }

                    h1 class="text-4xl md:text-5xl font-extrabold mb-6 leading-tight tracking-tight text-white" {
                        (props.title)
                    }

                    p class="text-lg md:text-xl text-slate-400 leading-relaxed max-w-2xl mx-auto" {
                        (props.description)
                    }
                }
            }
        }
    }
}
