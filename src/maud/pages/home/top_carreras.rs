use maud::{Markup, html};


pub struct TopCarreraItem {
    pub title: String,
    pub total: u16,
    pub icon: Markup,
}
pub struct TopCarreraProps {
    pub items: Vec<TopCarreraItem>,
}



pub fn top_carrera(props: TopCarreraProps) -> Markup {
    html!(
        section class="py-20 bg-slate-950" {
            div  class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                div class="flex justify-between items-end mb-12" {
                    div {
                        h2 class="text-3xl font-bold mb-2" {
                            "Explora por Áreas"
                        }
                        p class="text-slate-400" {
                            "Las carreras con mayor demanda este mes."
                        }
                    }
                    a href="#" class="text-blue-400 font-medium hover:underline flex items-center" {
                        "Ver todas las áreas"
                    }
                }

                div class="grid grid-cols-2 md:grid-cols-3 gap-4" {
                    @for carrera in &props.items {
                        div class="bg-theme-glass p-6 rounded-2xl text-center hover-glow transition cursor-pointer group" {
                            div class="bg-blue-500/10 w-12 h-12 rounded-xl flex items-center justify-center mx-auto mb-4 group-hover:bg-blue-500/20 transition" {
                                (carrera.icon)
                            }
                            h3 class="font-semibold text-sm" {
                                (carrera.title)
                            }
                            p class="text-xs text-slate-500 mt-1" {
                                (carrera.total) " prácticas"
                            }
                        }
                    }
                }
            }
        }
    )
}
