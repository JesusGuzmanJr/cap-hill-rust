use {const_format::formatcp, leptos::*, leptos_meta::*, leptos_router::*};

type Date = chrono::NaiveDate;

const ORG_NAME: &str = "Cap Hill Rust";
const MEETUP_URL: &str = "https://www.meetup.com/Cap-Hill-Rust/";
const GITHUB_URL: &str = "https://github.com/JesusGuzmanJr/cap-hill-rust";

/// The site-root relative folder where all compiled output is written to by
/// leptos.
pub const COMPILED_ASSETS: &str = "pkg";

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Stylesheet href="https://cdn.simplecss.org/simple.min.css" />
        <Link rel="manifest" href=formatcp!("{COMPILED_ASSETS}/manifest.json") />
        <Meta property="og:title" content=ORG_NAME />
        <Meta name="description" content="Cap Hill Rust is a Seattle based Meetup group to code/hack/learn Rust." />
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    let level = if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    };
    wasm_bindgen::UnwrapThrowExt::unwrap_throw(console_log::init_with_level(level));
    mount_to_body(App);
}
