use maud::{Markup, html};

pub fn left_home(children: Option<Markup>) -> Markup {
    html! {
        div class="flex flex-col gap-2 bg-dark-card h-full p-4 rounded-xl lg:overflow-auto" {
            div class="flex items-center gap-2" {
                img src="/public/images/logo.png" alt="Logo" class="size-24" { }
                h2 class="text-2xl text-white" translate="no" {
                    span class="text-rose-500 font-bold" { "P" } "rácticas " br; span class="text-rose-500 font-bold" { "P" } "erú Pro"
                }
            }
            h1 class="text-4xl font-bold text-rose-500" {
                "Convocatorias de prácticas" br; "pre y profesionales en el Perú"
            }
            p class="text-lg text-slate-300" {
                "Encuentra prácticas pre y profesionales. Empieza a construir tu futuro hoy."
            }
            div class="flex flex-col gap-2" {
                p class="text-slate-300 font-bold" { "Siguenos:" }
                ul class="flex gap-2 flex-wrap" {
                    li {
                        a class="text-slate-50 block p-2 rounded-md bg-slate-800 hover:bg-slate-700" {
                            svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="32"
                                height="32"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            {
                                path stroke="none" d="M0 0h24v24H0z" fill="none" {}
                                path d="M7 10v4h3v7h4v-7h3l1 -4h-4v-2a1 1 0 0 1 1 -1h3v-4h-3a5 5 0 0 0 -5 5v2h-3" {}
                            }
                            span class="hidden" { "Facebook" }
                        }
                    }
                    li {
                        a class="text-slate-50 block p-2 rounded-md bg-slate-800 hover:bg-slate-700" {
                            svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="32"
                                height="32"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            {
                                path stroke="none" d="M0 0h24v24H0z" fill="none" {}
                                path d="M21 7.917v4.034a9.948 9.948 0 0 1 -5 -1.951v4.5a6.5 6.5 0 1 1 -8 -6.326v4.326a2.5 2.5 0 1 0 4 2v-11.5h4.083a6.005 6.005 0 0 0 4.917 4.917" {}
                            }
                            span class="hidden" { "TikTok" }
                        }
                    }
                    li {
                        a class="text-slate-50 block p-2 rounded-md bg-slate-800 hover:bg-slate-700" {
                            svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="32"
                                height="32"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            {
                                path stroke="none" d="M0 0h24v24H0z" fill="none" {}
                                path d="M4 8a4 4 0 0 1 4 -4h8a4 4 0 0 1 4 4v8a4 4 0 0 1 -4 4h-8a4 4 0 0 1 -4 -4l0 -8" {}
                                path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" {}
                                path d="M16.5 7.5v.01" {}
                            }
                            span class="hidden" { "Instagram" }
                        }
                    }
                }
            }

            @if let Some(children) = children {
                (children)
            }
        }
    }
}
