use {
    crate::{components::*, path},
    leptos::*,
    leptos_meta::*,
};

// TODO: add library code here

#[component]
pub fn Library() -> impl IntoView {
    view! {
        <Meta name="description" content="Cap Hill Rust is a Seattle based Meetup group to code/hack/learn Rust." />
        <header>
            <h1>{crate::ORG_NAME}</h1>
            <p>"Welcome to the Cap Hill Rust Coding/Hacking/Learning homepage!"</p>
            <Nav/>
        </header>

        <main>
            <p>"Calling all Seattle-based Rust enthusiasts! We're an informal bunch getting together to program in Rust."</p>
            <p>"Join us to explore coding practices, share knowledge, and develop new skills together. Come meet like-minded individuals in a supportive and engaging environment to expand your programming horizons. Let's code, hack, and learn together!"</p>
            <p>"Checkout our "<a href=path::MEETUP>"Meetup\u{a0}page"</a>" for more information and to RSVP for our next event."</p>
        </main>
    }
}
