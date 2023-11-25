use {leptos::*, leptos_meta::*, leptos_router::*};

mod components;
mod pages;

const ORG_NAME: &str = "Cap Hill Rust";

pub mod path {
    pub const HOME: &str = "/";
    pub const MEETUP: &str = "https://www.meetup.com/Cap-Hill-Rust/";
    pub const GITHUB: &str = "https://github.com/JesusGuzmanJr/cap-hill-rust";
    pub const LIBRARY: &str = "/library";
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Stylesheet href="https://cdn.simplecss.org/simple.min.css"/>
        <Link rel="manifest" href="/manifest.json"/>
        <Meta property="og:title" content=ORG_NAME/>
        <Router>
            <Routes>
                <Route path=path::HOME view=pages::Index/>
                <Route path=path::LIBRARY view=pages::Library/>
            </Routes>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    wasm_bindgen::UnwrapThrowExt::unwrap_throw(console_log::init_with_level(
        if cfg!(debug_assertions) {
            log::Level::Debug
        } else {
            log::Level::Info
        },
    ));

    mount_to_body(App);
}
