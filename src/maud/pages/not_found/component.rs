use maud::{Markup, html};

pub struct NotFoundComponentProps<'t> {
    pub title: &'t str,
    pub description: &'t str,
}

pub fn not_found_component(props: NotFoundComponentProps) -> Markup {
    html! {
        section class="relative min-h-[70vh] flex flex-col justify-center items-center py-20 px-6 overflow-hidden bg-slate-950 text-center" {
            // Background glowing blur elements in Rose/Pink color tones
            div class="absolute -top-24 -left-24 w-96 h-96 bg-rose-600/10 rounded-full blur-3xl pointer-events-none" { }
            div class="absolute -bottom-24 -right-24 w-96 h-96 bg-rose-900/10 rounded-full blur-3xl pointer-events-none" { }

            div class="relative z-10 max-w-xl mx-auto flex flex-col items-center" {
                // Interactive Radar / Scanning search effect in Rose color
                div class="relative w-32 h-32 mb-8 flex items-center justify-center" {
                    // Outer pulse animation
                    div class="absolute inset-0 rounded-full border border-rose-500/20 animate-ping" { }
                    // Middle pulse animation
                    div class="absolute inset-4 rounded-full border border-rose-500/10 animate-pulse" { }
                    // Inner circle/glow containing the search glass icon
                    div class="absolute inset-8 rounded-full bg-rose-500/10 flex items-center justify-center border border-rose-500/30 shadow-[0_0_20px_rgba(244,63,94,0.15)]" {
                        svg class="w-10 h-10 text-rose-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5" {
                            path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.637 10.637z" { }
                        }
                    }
                }

                // Styled 404 Text with Rose/Pink gradient and text glow
                h1 class="text-8xl md:text-9xl font-black tracking-tighter bg-linear-to-r from-rose-400 via-rose-500 to-pink-500 bg-clip-text text-transparent drop-shadow-[0_0_30px_rgba(244,63,94,0.25)] mb-4 select-none animate-pulse" {
                    "404"
                }

                // Title passed by properties
                h2 class="text-2xl md:text-3xl font-bold text-white mb-4 tracking-tight" {
                    (props.title)
                }

                // Description passed by properties
                p class="text-slate-400 mb-8 max-w-md mx-auto leading-relaxed" {
                    (props.description)
                }

                // Action Navigation Buttons
                div class="flex flex-col sm:flex-row gap-4 w-full sm:w-auto" {
                    a href="/" class="inline-flex items-center justify-center bg-rose-600 hover:bg-rose-700 text-white font-semibold py-3 px-8 rounded-xl transition duration-300 transform hover:-translate-y-0.5 hover:shadow-lg hover:shadow-rose-500/25 focus:outline-none focus:ring-2 focus:ring-rose-500 focus:ring-offset-2 focus:ring-offset-slate-950 gap-2 text-sm" {
                        svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                            path stroke-linecap="round" stroke-linejoin="round" d="M10 19l-7-7m0 0l7-7m-7 7h18" { }
                        }
                        "Volver al inicio"
                    }
                    a href="/busqueda" class="inline-flex items-center justify-center bg-slate-900 hover:bg-slate-800 text-slate-300 hover:text-white border border-slate-800 hover:border-rose-500/30 font-semibold py-3 px-8 rounded-xl transition duration-300 transform hover:-translate-y-0.5 focus:outline-none focus:ring-2 focus:ring-rose-500 focus:ring-offset-2 focus:ring-offset-slate-950 gap-2 text-sm" {
                        svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                            path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" { }
                        }
                        "Buscar Prácticas"
                    }
                }
            }
        }
    }
}
