use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::navbar::NavBar;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <NavBar/>
        <main>
          <Routes fallback=NotFound>
            <Route path=path!("/") view=Home/>
          </Routes>
        </main>
      </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div>
        Hello World!
      </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
      <div class="text-red-500">
        We couldnt find the page youre looking for!
      </div>
    }
}
