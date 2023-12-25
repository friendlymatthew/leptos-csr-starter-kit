use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::html::Div;
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/standard.css"/>

        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/:else" view=ErrorPage/>
            </Routes>
        </Router>
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

    view! {
        <main class="min-h-screen py-20 w-full space-y-8">
            <div class="text-center font-robotomono space-y-2">
                <p class="text-center font-robotomono text-3xl">Welcome to Leptos!</p>
                <p>Set up a client side rendered Leptos app with one command</p>
            </div>
            <div class="flex space-x-20 justify-center">
                {LEPTOS_HELPFUL
                    .into_iter()
                    .enumerate()
                    .map(|(idx, (link, title))| {
                        view! {
                            <a class: bg - color - 1 = move || idx == 0 class : bg - color - 2 =
                            move || idx != 0 href = { link } target = "_blank" rel = "noreferrer" >
                            < p > { title } </ p > </ a >
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <div
                node_ref=el
                class="font-robotomono px-4 py-2 space-y-4 border border-gray-400/30 rounded shadow hover:shadow-lg fixed select-none cursor-move z-24"
                style=move || format!("touch-action: none; {}", style())
            >
                <div>
                    <p>
                        Contribute to the
                        <a
                            class="hover:no-underline underline"
                            href="https://github.com/friendlymatthew/create-leptos-csr"
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
                            href="https://github.com/friendlymatthew/create-leptos-csr/issues"
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
        <main class=format!("h-screen w-full flex flex-col items-center justify-center")>
            <p class="">Unknown command: {unknown}</p>
        </main>
    }
}
