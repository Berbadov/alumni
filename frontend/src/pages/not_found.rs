use leptos::prelude::*;

use crate::components::NavLink;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="hero">
            <h1>"404 — Page not found"</h1>
            <p>"The page you are looking for does not exist."</p>
            <div class="actions">
                <NavLink href="/" class="btn btn-primary">"Back to home"</NavLink>
            </div>
        </section>
    }
}
