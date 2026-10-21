mod api;
mod components;
mod pages;

use leptos::ev;
use leptos::prelude::*;

use components::{Layout, Navigator, current_path};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Route {
    Home,
    Alumni,
    Jobs,
    JobDetail(u64),
    Applications,
    About,
    NotFound,
}

fn parse_route(path: &str) -> Route {
    let path = path.trim_end_matches('/');
    match path {
        "" | "/" => Route::Home,
        "/alumni" => Route::Alumni,
        "/jobs" => Route::Jobs,
        "/applications" => Route::Applications,
        "/about" => Route::About,
        _ => path
            .strip_prefix("/jobs/")
            .and_then(|id| id.parse::<u64>().ok())
            .map_or(Route::NotFound, Route::JobDetail),
    }
}

#[component]
fn App() -> impl IntoView {
    let path = RwSignal::new(current_path());
    provide_context(Navigator { path });

    let handle = window_event_listener(ev::popstate, move |_| path.set(current_path()));
    on_cleanup(move || handle.remove());

    view! {
        <Layout>
            {move || match parse_route(&path.get()) {
                Route::Home => view! { <pages::Home /> }.into_any(),
                Route::Alumni => view! { <pages::Alumni /> }.into_any(),
                Route::Jobs => view! { <pages::Jobs /> }.into_any(),
                Route::JobDetail(id) => view! { <pages::JobDetail job_id=id /> }.into_any(),
                Route::Applications => view! { <pages::Applications /> }.into_any(),
                Route::About => view! { <pages::About /> }.into_any(),
                Route::NotFound => view! { <pages::NotFound /> }.into_any(),
            }}
        </Layout>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
