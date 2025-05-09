use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::navbar::NavBar;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Logo />
            <NavBar />
            <main>
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Logo() -> impl IntoView {
    view! {
        <div class="flex font-mono text-4xl pt-6 pl-6">
            <div class="text-dark-blue">open</div>
            <div class="dark-gray">erase</div>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>Hello World!</div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div>
            We couldnt find the page youre looking for!
        </div>
    }
}
