use leptos::prelude::*;

use crate::components::NavLink;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <section class="hero">
            <h1>"Alumni Tracking System"</h1>
            <p>
                "Web Development, Fall 2026–2027. Browse classmates, their degrees, "
                "and graduation years."
            </p>
            <div class="actions">
                <NavLink href="/alumni" class="btn btn-primary">"Browse alumni"</NavLink>
                <NavLink href="/jobs" class="btn btn-secondary">"View job postings"</NavLink>
            </div>
        </section>
    }
}
