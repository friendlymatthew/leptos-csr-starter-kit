use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::use_interval_fn;
pub const BACKGROUND_CSS: &str =
    "text-white w-screen h-screen flex flex-col justify-start items-center";
pub const PURPLE_GRADIENT: &str = "bg-gradient-to-br from-[#12172a] to-purple-900";
pub const BUTTON_BACKGROUND_CSS: &str = "flex items-center text-center px-8 py-2 rounded-3xl border-2 border-white hover:text-indigo-800 transition ease-in duration-300 hover:bg-white";

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-csr-tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/:thing" view=ErrorPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Button(href: &'static str, title: &'static str) -> impl IntoView {
    view! {
        <a
            class=format!("inline-block font-medium {}", BUTTON_BACKGROUND_CSS)
            target="_blank"
            href=href
            rel="noreferrer"
        >
            <p>{title}</p>
        </a>
    }
}

#[component]
fn CommandLine() -> impl IntoView {
    let cargo_install = "cargo add create-leptos-csr-tw";

    let (display_text, set_display_text) = create_signal(String::new());
    let (index, set_index) = create_signal(0);
    let (deleting, set_deleting) = create_signal(false);
    let (pause_duration, set_pause_duration) = create_signal(0_u64);
    let update_text = move || {
        if pause_duration.get() > 0 {
            set_pause_duration.set(pause_duration.get() - 100);
            return;
        }

        let curr_idx = index.get();

        if !deleting.get() {
            if curr_idx < cargo_install.len() {
                set_index.set(curr_idx + 1);
                set_display_text.set(cargo_install.chars().take(curr_idx + 1).collect());
            } else {
                set_pause_duration.set(5_000);
                set_deleting.set(true);
            }
        } else {
            if curr_idx > 0 {
                set_index.set(curr_idx - 1);
                set_display_text.set(cargo_install.chars().take(curr_idx - 1).collect());
            } else {
                set_deleting.set(false);
            }
        }
    };
    use_interval_fn(update_text, 100);
    view! {
        <a
            target="_blank"
            rel="noreferrer"
            href="https://crates.io/crates/create-leptos-csr-tw"
            class=format!("{} text-2xl md:text-3xl font-robotomono", BUTTON_BACKGROUND_CSS)
        >
            <p>{"> "} {move || display_text.get()}</p>
        </a>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let sizing = "w-10/12 md:w-2/3";

    view! {
        <main class=format!(
            "{} {} space-y-[4rem] font-opensans py-20",
            BACKGROUND_CSS,
            PURPLE_GRADIENT,
        )>
            <div class=format!("{} text-center", sizing)>
                <p class="text-4xl md:text-6xl font-extrabold">
                    {"A "} <span class="italic">blazingly fast</span> {" way to start a "}
                    <span class="text-fuchsia-400">Leptos</span> {" "}
                    <span class="text-emerald-400">client-side rendered</span> {" app with "}
                    <span class="text-sky-500">tailwind</span>

                </p>
            </div>
            <div class=format!("{} text-xl md:text-2xl flex justify-center space-x-[5rem]", sizing)>
                <Button
                    href="https://github.com/friendlymatthew/leptos-csr-tailwind"
                    title="See boilerplate"
                />
                <Button
                    href="https://github.com/friendlymatthew/create-leptos-csr-tw"
                    title="Usage"
                />
            </div>
            <div class=format!("{} flex justify-center", sizing)>
                <CommandLine/>
            </div>
        </main>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || params.with(|p| p.get("thing").cloned().unwrap_or_default());

    let unknown = p_unknown();

    view! {
        <main class=format!(
            "{} h-screen w-full flex flex-col items-center justify-center",
            BACKGROUND_CSS,
        )>
            <p class="">Unknown command: {unknown}</p>
        </main>
    }
}
