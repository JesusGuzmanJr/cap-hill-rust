use {crate::path::*, leptos::*, leptos_router::*};

#[component]
pub fn Nav() -> impl IntoView {
    let current = |path| {
        if use_route().path() == path {
            "current"
        } else {
            ""
        }
    };
    view! {
        <nav>
            <ul>
                <li><A class=move || current(HOME) href=HOME>Home</A></li>
                <li><a href=MEETUP>Meetup</a></li>
                <li><A class=move || current(LIBRARY) href=LIBRARY>Library</A></li>
                <li><a href=GITHUB>GitHub</a></li>
            </ul>
        </nav>
    }
}
