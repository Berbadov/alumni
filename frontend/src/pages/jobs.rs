use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::{self, JobPosting};
use crate::components::NavLink;

#[component]
pub fn Jobs() -> impl IntoView {
    let postings = RwSignal::new(Vec::<JobPosting>::new());
    let loading = RwSignal::new(true);
    let error = RwSignal::new(None::<String>);
    let search = RwSignal::new(String::new());
    let employment_type = RwSignal::new(String::new());
    let active_only = RwSignal::new(true);

    let load = move || {
        loading.set(true);
        error.set(None);
        spawn_local(async move {
            match api::fetch_job_postings().await {
                Ok(records) => postings.set(records),
                Err(message) => error.set(Some(message)),
            }
            loading.set(false);
        });
    };
    Effect::new(move |_| load());

    let types = move || {
        let mut types: Vec<String> = postings
            .get()
            .into_iter()
            .map(|posting| posting.employment_type)
            .filter(|employment_type| !employment_type.is_empty())
            .collect();
        types.sort();
        types.dedup();
        types
    };

    let filtered = move || {
        let query = search.get().trim().to_lowercase();
        let selected_type = employment_type.get();
        postings
            .get()
            .into_iter()
            .filter(|posting| {
                if active_only.get() && !posting.is_active {
                    return false;
                }
                if !selected_type.is_empty() && posting.employment_type != selected_type {
                    return false;
                }
                if query.is_empty() {
                    return true;
                }
                format!(
                    "{} {} {} {}",
                    posting.title, posting.company, posting.location, posting.employment_type
                )
                .to_lowercase()
                .contains(&query)
            })
            .collect::<Vec<_>>()
    };

    view! {
        <section class="stack">
            <div>
                <h1 class="page-title">"Job postings"</h1>
                <p class="page-subtitle">"Roles shared by the alumni community."</p>
            </div>

            <div class="card toolbar">
                <label class="field grow">
                    <span>"Search"</span>
                    <input
                        class="input"
                        type="search"
                        placeholder="Title, company, or location"
                        on:input=move |event| search.set(event_target_value(&event))
                    />
                </label>
                <label class="field">
                    <span>"Employment type"</span>
                    <select
                        class="select"
                        on:change=move |event| employment_type.set(event_target_value(&event))
                    >
                        <option value="">"All types"</option>
                        {move || {
                            types()
                                .into_iter()
                                .map(|kind| {
                                    let value = kind.clone();
                                    view! { <option value=value>{kind}</option> }
                                })
                                .collect_view()
                        }}
                    </select>
                </label>
                <label class="check">
                    <input
                        type="checkbox"
                        prop:checked=move || active_only.get()
                        on:change=move |event| active_only.set(event_target_checked(&event))
                    />
                    <span>"Active only"</span>
                </label>
            </div>

            {move || {
                if let Some(message) = error.get() {
                    view! { <p class="alert alert-error" role="alert">"Error: " {message}</p> }
                        .into_any()
                } else if loading.get() {
                    view! { <p class="muted">"Loading job postings…"</p> }.into_any()
                } else if filtered().is_empty() {
                    view! { <p class="muted">"No job postings match these filters."</p> }
                        .into_any()
                } else {
                    view! {
                        <div class="jobs-grid">
                            {filtered()
                                .into_iter()
                                .map(|posting| {
                                    let detail_href = format!("/jobs/{}", posting.id);
                                    let detail_href_again = detail_href.clone();
                                    let badge_class = if posting.is_active {
                                        "badge badge-active"
                                    } else {
                                        "badge badge-closed"
                                    };
                                    let badge_text = if posting.is_active {
                                        "Active"
                                    } else {
                                        "Closed"
                                    };
                                    view! {
                                        <article class="card job-card">
                                            <div class="job-card-head">
                                                <div>
                                                    <h2>
                                                        <NavLink href=detail_href class="job-link">
                                                            {posting.title.clone()}
                                                        </NavLink>
                                                    </h2>
                                                    <p class="company">{posting.company.clone()}</p>
                                                </div>
                                                <span class=badge_class>{badge_text}</span>
                                            </div>
                                            <p class="job-summary">{posting.description.clone()}</p>
                                            <ul class="job-meta">
                                                <li>{posting.location.clone()}</li>
                                                <li>{posting.employment_type.clone()}</li>
                                            </ul>
                                            <NavLink
                                                href=detail_href_again
                                                class="btn btn-secondary btn-small"
                                            >
                                                "View details"
                                            </NavLink>
                                        </article>
                                    }
                                })
                                .collect_view()}
                        </div>
                    }
                        .into_any()
                }
            }}
        </section>
    }
}
