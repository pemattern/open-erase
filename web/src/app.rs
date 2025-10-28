use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::{login::Login, navbar::NavBar};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <Router>
                <NavBar/>
                <main class="w-full rounded-md bg-light-gray p-4 m-4">
                    <Routes fallback=NotFound>
                        <Route path=path!("") view=Home/>
                        <Route path=path!("login") view=Login/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>"Hello World!"</div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div>
            "We couldnt find the page youre looking for!"
        </div>
    }
}
