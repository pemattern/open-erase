use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <nav>
          <a href="/">Home</a>
        </nav>
        <main>
          <Routes fallback=NotFound>
            <Route path=path!("/") view=Home/>
          </Routes>
        </main>
      </Router>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
      <nav>
        <ul>
        </ul>
      </nav>
    }
}

#[component]
pub fn NavBarEntry(path: &'static str, text: &'static str) -> impl IntoView {
    view! {
      <A href=path>{text}</A>
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
