use leptos::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::window;

#[derive(Clone, Copy)]
pub struct Navigator {
    pub path: RwSignal<String>,
}

impl Navigator {
    pub fn go(self, href: &str) {
        self.path.set(href.to_owned());
        if let Some(window) = window() {
            let _ = window
                .history()
                .and_then(|history| history.push_state_with_url(&JsValue::NULL, "", Some(href)));
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
    }

    pub fn is_active(self, href: &str) -> bool {
        let current = self.path.get();
        current == href || (href != "/" && current.starts_with(&format!("{href}/")))
    }
}

pub fn current_path() -> String {
    window()
        .and_then(|window| window.location().pathname().ok())
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| "/".to_owned())
}

#[component]
pub fn NavLink(
    #[prop(into)] href: String,
    #[prop(default = "nav-link")] class: &'static str,
    children: Children,
) -> impl IntoView {
    let navigator = expect_context::<Navigator>();
    let class_name = class.to_owned();
    let active_href = href.clone();
    let click_href = href.clone();
    view! {
        <a
            href=href
            class=move || {
                if navigator.is_active(&active_href) {
                    format!("{class_name} active")
                } else {
                    class_name.clone()
                }
            }
            on:click=move |event| {
                event.prevent_default();
                navigator.go(&click_href);
            }
        >
            {children()}
        </a>
    }
}

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="site">
            <header class="site-header">
                <div class="container header-inner">
                    <nav class="nav" aria-label="Main">
                        <NavLink href="/" class="brand">"Alumni Tracker"</NavLink>
                        <NavLink href="/alumni">"Alumni"</NavLink>
                        <NavLink href="/jobs">"Jobs"</NavLink>
                        <NavLink href="/applications">"Applications"</NavLink>
                        <NavLink href="/about">"About"</NavLink>
                    </nav>
                    <ThemeToggle />
                </div>
            </header>
            <main class="container site-main">{children()}</main>
            <footer class="site-footer">
                <div class="container">
                    "Alumni Tracking System — Web Development, Fall 2026–2027"
                </div>
            </footer>
        </div>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let dark = RwSignal::new(
        window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
            .is_some_and(|root| root.class_list().contains("dark")),
    );

    Effect::new(move |_| {
        let is_dark = dark.get();
        if let Some(root) = window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
        {
            let _ = root.class_list().toggle_with_force("dark", is_dark);
        }
        if let Some(storage) = window()
            .and_then(|window| window.local_storage().ok())
            .flatten()
        {
            let _ = storage.set_item("alumni-theme", if is_dark { "dark" } else { "light" });
        }
    });

    view! {
        <button
            type="button"
            class="theme-toggle"
            aria-label="Toggle theme"
            on:click=move |_| dark.update(|value| *value = !*value)
        >
            {move || if dark.get() { "Light mode" } else { "Dark mode" }}
        </button>
    }
}

pub fn format_time(value: &str) -> String {
    value
        .get(..16)
        .map_or_else(|| value.to_owned(), |short| short.replace('T', " "))
}
