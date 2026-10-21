use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::{self, APPLICATION_STATUSES, Alumni, JobApplication, JobPosting};
use crate::components::format_time;

fn snippet(text: &str) -> String {
    if text.chars().count() > 90 {
        format!("{}…", text.chars().take(90).collect::<String>())
    } else {
        text.to_owned()
    }
}

#[component]
pub fn Applications() -> impl IntoView {
    let applications = RwSignal::new(Vec::<JobApplication>::new());
    let alumni = RwSignal::new(Vec::<Alumni>::new());
    let postings = RwSignal::new(Vec::<JobPosting>::new());
    let posting_filter = RwSignal::new(String::new());
    let alumni_filter = RwSignal::new(String::new());
    let status_filter = RwSignal::new(String::new());
    let loading = RwSignal::new(true);
    let error = RwSignal::new(None::<String>);
    let feedback = RwSignal::new(None::<(bool, String)>);

    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(records) = api::fetch_alumni().await {
                alumni.set(records);
            }
            if let Ok(records) = api::fetch_job_postings().await {
                postings.set(records);
            }
        });
    });

    let load = move || {
        loading.set(true);
        error.set(None);
        let posting = posting_filter.get().parse::<u64>().ok();
        let alumnus = alumni_filter.get().parse::<u64>().ok();
        let status = status_filter.get();
        spawn_local(async move {
            let status = if status.is_empty() {
                None
            } else {
                Some(status)
            };
            match api::fetch_applications(posting, alumnus, status.as_deref()).await {
                Ok(records) => applications.set(records),
                Err(message) => error.set(Some(message)),
            }
            loading.set(false);
        });
    };
    Effect::new(move |_| load());

    let update_status = move |id: u64, status: String| {
        feedback.set(None);
        spawn_local(async move {
            match api::update_application_status(id, &status).await {
                Ok(_) => feedback.set(Some((true, format!("Application #{id} marked {status}.")))),
                Err(message) => feedback.set(Some((false, message))),
            }
            load();
        });
    };

    view! {
        <section class="stack">
            <div>
                <h1 class="page-title">"Job applications"</h1>
                <p class="page-subtitle">"Track applications and update their status."</p>
            </div>

            <div class="card toolbar">
                <label class="field">
                    <span>"Job posting"</span>
                    <select
                        class="select"
                        on:change=move |event| {
                            posting_filter.set(event_target_value(&event));
                            load();
                        }
                    >
                        <option value="">"All postings"</option>
                        {move || {
                            postings
                                .get()
                                .into_iter()
                                .map(|posting| {
                                    view! {
                                        <option value=posting.id.to_string()>
                                            {format!("#{} {}", posting.id, posting.title)}
                                        </option>
                                    }
                                })
                                .collect_view()
                        }}
                    </select>
                </label>
                <label class="field">
                    <span>"Alumni"</span>
                    <select
                        class="select"
                        on:change=move |event| {
                            alumni_filter.set(event_target_value(&event));
                            load();
                        }
                    >
                        <option value="">"All alumni"</option>
                        {move || {
                            alumni
                                .get()
                                .into_iter()
                                .map(|person| {
                                    view! {
                                        <option value=person.id.to_string()>
                                            {format!("#{} {}", person.id, person.full_name)}
                                        </option>
                                    }
                                })
                                .collect_view()
                        }}
                    </select>
                </label>
                <label class="field">
                    <span>"Status"</span>
                    <select
                        class="select"
                        on:change=move |event| {
                            status_filter.set(event_target_value(&event));
                            load();
                        }
                    >
                        <option value="">"All statuses"</option>
                        {APPLICATION_STATUSES
                            .into_iter()
                            .map(|status| view! { <option value=status>{status}</option> })
                            .collect_view()}
                    </select>
                </label>
            </div>

            {move || {
                feedback
                    .get()
                    .map(|(success, message)| {
                        let class = if success {
                            "alert alert-success"
                        } else {
                            "alert alert-error"
                        };
                        view! { <p class=class role="status">{message}</p> }
                    })
            }}

            {move || {
                if let Some(message) = error.get() {
                    view! { <p class="alert alert-error" role="alert">"Error: " {message}</p> }
                        .into_any()
                } else if loading.get() {
                    view! { <p class="muted">"Loading applications…"</p> }.into_any()
                } else if applications.get().is_empty() {
                    view! { <p class="muted">"No applications match these filters."</p> }
                        .into_any()
                } else {
                    view! {
                        <div class="card table-wrap">
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th>"Alumni"</th>
                                        <th>"Job posting"</th>
                                        <th>"Cover letter"</th>
                                        <th>"Status"</th>
                                        <th>"Updated"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || {
                                        applications
                                            .get()
                                            .into_iter()
                                            .map(|application| {
                                                let id = application.id;
                                                let posting_id = application.job_posting_id;
                                                let alumni_id = application.alumni_id;
                                                let cover_letter = application.cover_letter.clone();
                                                let cover_hint = application.cover_letter.clone();
                                                let status_value = application.status.clone();
                                                let updated = format_time(&application.updated_at);
                                                let alumni_name = move || {
                                                    alumni
                                                        .get()
                                                        .into_iter()
                                                        .find(|person| person.id == alumni_id)
                                                        .map_or_else(
                                                            || format!("#{alumni_id}"),
                                                            |person| person.full_name,
                                                        )
                                                };
                                                let posting_title = move || {
                                                    postings
                                                        .get()
                                                        .into_iter()
                                                        .find(|posting| posting.id == posting_id)
                                                        .map_or_else(
                                                            || format!("#{posting_id}"),
                                                            |posting| posting.title,
                                                        )
                                                };
                                                view! {
                                                    <tr>
                                                        <td class="strong">{alumni_name}</td>
                                                        <td>{posting_title}</td>
                                                        <td class="cover-cell" title=cover_hint>
                                                            {snippet(&cover_letter)}
                                                        </td>
                                                        <td>
                                                            <select
                                                                class="select status-select"
                                                                prop:value=status_value
                                                                on:change=move |event| {
                                                                    update_status(
                                                                        id,
                                                                        event_target_value(&event),
                                                                    )
                                                                }
                                                            >
                                                                {APPLICATION_STATUSES
                                                                    .into_iter()
                                                                    .map(|status| {
                                                                        view! { <option value=status>{status}</option> }
                                                                    })
                                                                    .collect_view()}
                                                            </select>
                                                        </td>
                                                        <td class="nowrap">{updated}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect_view()
                                    }}
                                </tbody>
                            </table>
                        </div>
                        <p class="muted">
                            {move || {
                                let count = applications.get().len();
                                format!("{count} application(s)")
                            }}
                        </p>
                    }
                        .into_any()
                }
            }}
        </section>
    }
}
