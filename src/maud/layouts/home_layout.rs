use maud::{Markup, html};

use crate::maud::components::footer::footer;

pub fn home_layout(left: Markup, right: Markup) -> Markup {
    html! {
        section class="grid grid-cols-1 gap-4 lg:gap-0 lg:grid-cols-[384px_1fr]" {
            div class="flex-1 lg:h-full p-2 w-full lg:fixed lg:w-96" {
                (left)
            }
            div class="flex-1 p-2 lg:col-start-2" {
                (right)
                br;
                (footer())
            }
        }
    }
}
