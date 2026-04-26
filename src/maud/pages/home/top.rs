use maud::{Markup, html};

pub struct TopItem {
    pub title: String,
    pub total: u16,
    pub icon: Markup,
}
pub struct TopProps {
    pub items: Vec<TopItem>,
    pub title: &'static str,
    pub description: &'static str,
    pub link: &'static str,
    pub link_text: &'static str,
}

pub fn top(props: TopProps) -> Markup {
    html!(
        section class="py-20 bg-slate-950" {
            div  class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                div class="flex justify-between items-end mb-12 text-lg" {
                    div {
                        h2 class="text-3xl font-bold mb-2" {
                            (props.title)
                        }
                        p class="text-slate-400" {
                            (props.description)
                        }
                    }
                    a href=(props.link) class="text-blue-400 font-medium hover:underline flex items-center" {
                        (props.link_text)
                    }
                }

                div class="grid grid-cols-2 md:grid-cols-3 gap-4 text-lg" {
                    @for item in &props.items {
                        div class="bg-theme-glass p-6 rounded-2xl text-center hover-glow transition cursor-pointer group" {
                            div class="bg-blue-500/10 w-12 h-12 rounded-xl flex items-center justify-center mx-auto mb-4 group-hover:bg-blue-500/20 transition" {
                                (item.icon)
                            }
                            h3 class="font-semibold" {
                                (item.title)
                            }
                            p class="text-base text-slate-500 mt-1" {
                                (item.total) " prácticas"
                            }
                        }
                    }
                }
            }
        }
    )
}
