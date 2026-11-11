use maud::{Markup, html};

use super::layout::page;

pub fn not_found(path: &str) -> Markup {
    page(
        "404 - Page not found",
        &html! {
            p { "No route matches " code { (path) } "." }
            p { a href="/" { "Back to home" } }
        },
    )
}
