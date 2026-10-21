use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::{self, Alumni, JobPosting};
use crate::components::{NavLink, format_time};

#[component]
pub fn JobDetail(job_id: u64) -> impl IntoView {
    let posting = RwSignal::new(None::<JobPosting>);
    let alumni = RwSignal::new(Vec::<Alumni>::new());
    let loading = RwSignal::new(true);
    let error = RwSignal::new(None::<String>);
    let selected_alumni = RwSignal::new(String::new());
    let cover_letter = RwSignal::new(String::new());
    let feedback = RwSignal::new(None::<(bool, String)>);
    let submitting = RwSignal::new(false);

    let load = move || {
        loading.set(true);
        error.set(None);
        spawn_local(async move {
            match api::fetch_job_posting(job_id).await {
                Ok(record) => posting.set(Some(record)),
                Err(message) => error.set(Some(message)),
            }
            match api::fetch_alumni().await {
                Ok(records) => {
                    if selected_alumni.get().is_empty() {
                        if let Some(first) = records.first() {
                            selected_alumni.set(first.id.to_string());
                        }
                    }
                    alumni.set(records);
                }
                Err(message) => error.set(Some(message)),
            }
            loading.set(false);
        });
    };
    Effect::new(move |_| load());

    let submit = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();
        if submitting.get() {
            return;
        }
        let Ok(alumni_id) = selected_alumni.get().parse::<u64>() else {
            feedback.set(Some((false, "Choose an alumnus to apply.".to_owned())));
            return;
        };
        let letter = cover_letter.get();
        if letter.trim().is_empty() {
            feedback.set(Some((false, "Write a short cover letter.".to_owned())));
            return;
        }
        submitting.set(true);
        feedback.set(None);
        spawn_local(async move {
            match api::create_application(job_id, alumni_id, &letter).await {
                Ok(_) => {
                    cover_letter.set(String::new());
                    feedback.set(Some((
                        true,
                        "Application submitted successfully.".to_owned(),
                    )));
                }
                Err(message) => feedback.set(Some((false, message))),
            }
            submitting.set(false);
        });
    };

    view! {
        <section class="stack">
            <NavLink href="/jobs" class="back-link">"← Back to job postings"</NavLink>

            {move || {
                if let Some(message) = error.get() {
                    view! { <p class="alert alert-error" role="alert">"Error: " {message}</p> }
                        .into_any()
                } else if loading.get() {
                    view! { <p class="muted">"Loading job posting…"</p> }.into_any()
                } else if let Some(posting) = posting.get() {
                    let is_active = posting.is_active;
                    let badge_class = if is_active {
                        "badge badge-active"
                    } else {
                        "badge badge-closed"
                    };
                    let badge_text = if is_active { "Active" } else { "Closed" };
                    view! {
                        <article class="card job-detail">
                            <div class="job-card-head">
                                <div>
                                    <h1>{posting.title.clone()}</h1>
                                    <p class="company">{posting.company.clone()}</p>
                                </div>
                                <span class=badge_class>{badge_text}</span>
                            </div>
                            <dl class="detail-grid">
                                <div>
                                    <dt>"Location"</dt>
                                    <dd>{posting.location.clone()}</dd>
                                </div>
                                <div>
                                    <dt>"Employment type"</dt>
                                    <dd>{posting.employment_type.clone()}</dd>
                                </div>
                                <div>
                                    <dt>"Posted"</dt>
                                    <dd>{format_time(&posting.created_at)}</dd>
                                </div>
                                <div>
                                    <dt>"Author"</dt>
                                    <dd>"Alumni #" {posting.author_id}</dd>
                                </div>
                            </dl>
                            <p class="job-description">{posting.description.clone()}</p>
                            <a
                                class="text-link"
                                href=posting.apply_url.clone()
                                target="_blank"
                                rel="noreferrer"
                            >
                                "Original listing ↗"
                            </a>
                        </article>

                        <div class="card apply-card">
                            <h2>"Apply for this role"</h2>
                            {if is_active {
                                view! {
                                    <form class="form-stack" on:submit=submit>
                                        <label class="field">
                                            <span>"Alumni"</span>
                                            <select
                                                class="select"
                                                prop:value=move || selected_alumni.get()
                                                on:change=move |event| {
                                                    selected_alumni.set(event_target_value(&event))
                                                }
                                            >
                                                {move || {
                                                    alumni
                                                        .get()
                                                        .into_iter()
                                                        .map(|person| {
                                                            view! {
                                                                <option value=person.id.to_string()>
                                                                    {person.full_name}
                                                                </option>
                                                            }
                                                        })
                                                        .collect_view()
                                                }}
                                            </select>
                                        </label>
                                        <label class="field">
                                            <span>"Cover letter"</span>
                                            <textarea
                                                class="textarea"
                                                rows="5"
                                                placeholder="Why are you a good fit?"
                                                prop:value=move || cover_letter.get()
                                                on:input=move |event| {
                                                    cover_letter.set(event_target_value(&event))
                                                }
                                            ></textarea>
                                        </label>
                                        <button
                                            class="btn btn-primary"
                                            type="submit"
                                            disabled=move || submitting.get()
                                        >
                                            {move || if submitting.get() {
                                                "Submitting…"
                                            } else {
                                                "Submit application"
                                            }}
                                        </button>
                                        {move || {
                                            feedback
                                                .get()
                                                .map(|(success, message)| {
                                                    let class = if success {
                                                        "alert alert-success"
                                                    } else {
                                                        "alert alert-error"
                                                    };
                                                    view! {
                                                        <p class=class role="status">
                                                            {message}
                                                        </p>
                                                    }
                                                })
                                        }}
                                    </form>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <p class="alert">"Applications are closed for this posting."</p>
                                }
                                    .into_any()
                            }}
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <p class="muted">"This job posting could not be found."</p> }
                        .into_any()
                }
            }}
        </section>
    }
}
