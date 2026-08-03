use maud::{Markup, html};

pub struct MenuItem {
    pub title: String,
    pub url: String,
    pub is_call_to_action: bool,
    pub target: Option<String>,
    pub sub_menu: Option<Vec<MenuItem>>,
}

pub fn header(list_menu: Vec<MenuItem>) -> Markup {
    html! {
        nav class="fixed top-0 w-full z-50 bg-theme-glass border-b border-slate-800" {
            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                div class="flex justify-between h-16 items-center" {
                    a href="/" class="flex items-center gap-2" {
                        img src="/public/images/favicon.svg" alt="Logo" class="size-10" { }
                        h2 class="text-lg text-white" translate="no" {
                            span class="text-rose-500 font-bold" { "P" } "rácticas " span class="text-rose-500 font-bold" { "P" } "erú Pro"
                        }
                    }
                    div class="hidden md:flex space-x-8 items-center text-sm font-medium" {
                        @for item in &list_menu {
                            @if let Some(sub_menu) = &item.sub_menu {
                                div class="group relative" {
                                    p class="transition text-slate-300 hover:text-white cursor-pointer" {
                                        (item.title)
                                    }

                                    ul class="hidden group-hover:block w-max absolute right-0 top-full z-10 p-4 rounded-lg space-y-2 bg-slate-950/95 border border-slate-800 shadow" {
                                        @for sub_item in sub_menu {
                                            li {
                                                a class="text-slate-300 hover:text-white transition-colors" href=(sub_item.url) target=(sub_item.target.clone().unwrap_or("".to_string()))  {
                                                    (sub_item.title)
                                                }
                                            }
                                        }
                                    }
                                }
                            } @else {
                                @if item.is_call_to_action {
                                    a class="bg-rose-600 hover:bg-rose-700 text-white px-4 py-2 rounded-full transition"  href=(item.url) target=(item.target.clone().unwrap_or("".to_string()))  {
                                        (item.title)
                                    }
                                } @else {
                                    a class="text-slate-300 hover:text-white transition-colors" href=(item.url) target=(item.target.clone().unwrap_or("".to_string()))  {
                                        (item.title)
                                    }
                                }
                            }
                        }
                    }

                    div class="block md:hidden" {
                        button class="rounded-sm bg-gray-100 p-2 text-gray-600 transition hover:text-gray-600/75 dark:bg-gray-800 dark:text-white dark:hover:text-white/75" onclick="handle_open_dialog('menu-header', true)" {
                            svg xmlns="http://www.w3.org/2000/svg"
                            class="size-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            stroke-width="2" {
                                path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M4 6h16M4 12h16M4 18h16";
                            }
                        }
                    }
                }
            }
        }

        dialog class="lg:hidden! bg-slate-800 backdrop:bg-black/50 h-max max-h-4/5 w-full max-w-full m-auto mb-0" id="menu-header" onclick="handle_close_header_dialog(event, 'menu-header')" data-evref="handle_close_header_dialog" {
            form class="flex justify-end p-4 md:p-6 sticky top-0 bg-slate-800" method="dialog" {
                button type="submit" class="text-white" onclick="handle_open_dialog('menu-header', false)" {
                    svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path stroke="none" d="M0 0h24v24H0z" fill="none" { }
                        path d="M18 6l-12 12" { }
                        path d="M6 6l12 12" { }
                    }

                }
            }

            nav class="p-4 md:p-6" {
                ul class="flex flex-col gap-6 text-lg" {
                    @for item in &list_menu {
                        @if let Some(sub_menu) = &item.sub_menu {
                            details class="group open:p-2 open:bg-slate-900 rounded-lg" {
                                summary class="list-none text-gray-500 transition hover:text-gray-500/75 dark:text-white dark:hover:text-white/75 group-open:underline cursor-pointer w-max" {
                                    (item.title)
                                }

                                ul class="text-base p-2 mt-2" {
                                    @for sub_item in sub_menu {
                                        li {
                                            a class="transition-colors text-white hover:text-white/75" href=(sub_item.url) target=(sub_item.target.clone().unwrap_or("".to_string()))  {
                                                (sub_item.title)
                                            }
                                        }
                                    }
                                }
                            }
                        } @else {
                            @if item.is_call_to_action {
                                a class="bg-rose-600 hover:bg-rose-700 text-white px-4 py-2 rounded-full transition w-max" href=(item.url) target=(item.target.clone().unwrap_or("".to_string()))  {
                                    (item.title)
                                }
                            } @else {
                                a class="transition-colors text-white hover:text-white/75" href=(item.url) target=(item.target.clone().unwrap_or("".to_string()))  {
                                    (item.title)
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}
