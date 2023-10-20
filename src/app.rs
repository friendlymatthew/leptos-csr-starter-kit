use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::global::{
    BACKGROUND_CSS, FLOWER_GRADIENT, TEXT_CSS
};
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-csr-tailwind.css"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/:thing" view=ErrorPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <main class=format!("{}", BACKGROUND_CSS)>
            <a
                class="cursor-pointer underline decoration-[0.3em] decoration-purple-800 hover:decoration-dotted"
                href="https://github.com/friendlymatthew/leptos-csr-tailwind"
                rel="noreferrer"
                target="_blank"
            >
            <p class=format!(
                "{} text-slate-800",
                TEXT_CSS,
            )>A leptos clientside boilerplate with tailwind</p>
            </a>
            <p class=format!("{} italic text-slate-800", TEXT_CSS)>BOTTOM TEXT</p>
        </main>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || {
        params.with(|p| p.get("thing").cloned().unwrap_or_default())
    };

    let unknown = p_unknown();

    view! {
        <main class=format!("{} {}", BACKGROUND_CSS, FLOWER_GRADIENT)>
            <p class=format!("{}", TEXT_CSS)>Unknown command: {unknown}</p>
        </main>
    }
}