use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::global::{BACKGROUND_CSS, FLOWER_GRADIENT, PURPLE_GRADIENT};
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
fn Button(
    href: String,
    title: String,
    css: Option<String>
)-> impl IntoView {

    let (underline_css, bg_css) = if let Some(css) = css {
        (
            format!("text-[{}] hover:underline decoration-[0.3rem]", css),
            format!("")
        )
    } else {
        (
            format!(""),
            format!("font-medium px-8 py-2 rounded-full border border-white text-white hover:text-indigo-800 transition ease-in duration-300 hover:bg-white")
        )
    };

    view! {
        <a
            class=format!("inline-block {}", bg_css)
            target="_blank"
            href=href
            rel="noreferrer"
        >
            <p class=format!("{}", underline_css)>{title}</p>
        </a>
    }
}

#[component]
fn HomePage() -> impl IntoView {

     view! {
         <main class=format!("{} {} py-20", BACKGROUND_CSS, PURPLE_GRADIENT)>
             <div class=format!("w-full flex flex-col items-center justify-center space-y-[10rem]")>
                 <div class="w-2/3 text-center">
                     <p class="text-white italic text-3xl md:text-6xl font-opensans font-extrabold">
                         {"A blazingly fast way to start a "}
                         <Button
                             href="https://leptos-rs.github.io/leptos/".to_string()
                             title="Leptos".to_string()
                             css=Some("#ff7733".to_string())
                         />
                        {" "}
                         <Button
                             href="https://leptos-rs.github.io/leptos/01_introduction.html".to_string()
                             title="client-side rendered".to_string()
                             css=Some("#33bbff".to_string())
                         />{" app with "}
                         <Button
                             href="https://github.com/leptos-rs/leptos/discussions/125".to_string()
                             title="tailwind.".to_string()
                             css=Some("#dd33ff".to_string())
                         />
                     </p>
                 </div>
                <div class="w-2/3 text-center text-xl md:text-2xl space-x-[5rem]">
                    <Button
                        href="https://github.com/friendlymatthew/leptos-csr-tailwind".to_string()
                        title="See boilerplate".to_string()
                        css=None
                    />
                    <Button
                        href="https://github.com/friendlymatthew/create-leptos-csr-tw".to_string()
                        title="Usage".to_string()
                        css=None
                    />
                </div>
             </div>
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
            <p class="">Unknown command: {unknown}</p>
        </main>
    }
}