use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::html::Div;
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use leptos_theme::ThemeProvider;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <ThemeProvider>
            <Router>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/:else" view=ErrorPage/>
                </Routes>
            </Router>
        </ThemeProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let el = create_node_ref::<Div>();

    let inner_width = use_window()
        .as_ref()
        .map(|w| w.inner_width().unwrap().as_f64().unwrap())
        .unwrap_or(0.0);

    let UseDraggableReturn { x, y, style, .. } = use_draggable_with_options(
        el,
        UseDraggableOptions::default()
            .initial_value(Position {
                x: inner_width / 2.3,
                y: 300.0,
            })
            .prevent_default(true),
    );

    const LEPTOS_HELPFUL: [(&'static str, &'static str); 2] = [
        ("https://book.leptos.dev", "Leptos Docs"),
        ("https://discord.gg/umUZ5Y8F7u", "Join the Official Discord"),
    ];

    const REPO: &'static str = "https://github.com/friendlymatthew/leptos-csr-starter-kit";

    view! {
        <main class="dark:bg-[#1a1a1a] bg-white dark:text-white h-screen py-20 w-full space-y-8 font-robotomono">
            <div class="text-center space-y-2">
                <p class="text-3xl">leptos-csr-starter-kit</p>
                <p>Set up a client side rendered Leptos app with one command</p>
            </div>
            <div class="flex space-x-20 justify-center">
                {LEPTOS_HELPFUL
                    .into_iter()
                    .enumerate()
                    .map(|(idx, (link, title))| {
                        view! {
                            < a class : bg - color - 1 = move || idx == 0 class : bg - color - 2 =
                            move || idx != 0 href = { link } target = "_blank" rel = "noreferrer" >
                            < p > { title } </ p > </ a >
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <div
                node_ref=el
                class="px-4 py-2 space-y-4 border border-gray-400/30 rounded shadow hover:shadow-lg fixed select-none cursor-move z-24"
                style=move || format!("touch-action: none; {}", style())
            >
                <div>
                    <p>
                        Contribute to the
                        <a
                            class="hover:no-underline underline"
                            href=REPO
                            target="_blank"
                            rel="noreferrer"
                        >
                            repository!
                        </a>
                    </p>

                    <p>
                        Or, submit
                        <a
                            class="hover:no-underline underline"
                            href=format!("{}/issues/new", REPO)
                            target="_blank"
                            rel="noreferrer"
                        >
                            a issue
                        </a>
                    </p>
                </div>
                <div class="text-sm flex justify-between">
                    I am {move || x().round()} , {move || y().round()}
                    <p class="opacity-100">"🙈🙉🙊"</p>
                </div>
            </div>
        </main>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || params.with(|p| p.get("else").cloned().unwrap_or_default());

    let unknown = p_unknown();

    view! {
        <main class=format!("h-screen w-full flex flex-col items-center justify-center font-robotomono")>
            <p class="">Unknown command: {unknown}</p>
        </main>
    }
}
