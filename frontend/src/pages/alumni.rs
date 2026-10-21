use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::{self, Alumni as AlumniRecord};

#[component]
pub fn Alumni() -> impl IntoView {
    let alumni = RwSignal::new(Vec::<AlumniRecord>::new());
    let loading = RwSignal::new(true);
    let error = RwSignal::new(None::<String>);

    let load = move || {
        loading.set(true);
        error.set(None);
        spawn_local(async move {
            match api::fetch_alumni().await {
                Ok(records) => alumni.set(records),
                Err(message) => error.set(Some(message)),
            }
            loading.set(false);
        });
    };
    Effect::new(move |_| load());

    view! {
        <section class="stack">
            <div>
                <h1 class="page-title">"Alumni"</h1>
                <p class="page-subtitle">"Web Development, Fall 2026–2027"</p>
            </div>

            {move || {
                if let Some(message) = error.get() {
                    view! { <p class="alert alert-error" role="alert">"Error: " {message}</p> }
                        .into_any()
                } else if loading.get() {
                    view! { <p class="muted">"Loading alumni…"</p> }.into_any()
                } else if alumni.get().is_empty() {
                    view! { <p class="muted">"No alumni yet."</p> }.into_any()
                } else {
                    view! {
                        <div class="card table-wrap">
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th>"Name"</th>
                                        <th>"Degree"</th>
                                        <th>"Graduation year"</th>
                                        <th>"Email"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || {
                                        alumni
                                            .get()
                                            .into_iter()
                                            .map(|person| {
                                                view! {
                                                    <tr>
                                                        <td class="strong">{person.full_name}</td>
                                                        <td>{person.degree}</td>
                                                        <td>{person.graduation_year}</td>
                                                        <td>{person.email}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect_view()
                                    }}
                                </tbody>
                            </table>
                        </div>
                    }
                        .into_any()
                }
            }}
        </section>
    }
}
