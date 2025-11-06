use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::{
    login::{AuthContext, AuthProvider, Login},
    navbar::NavBar,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <AuthProvider>
                <Routes fallback=NotFound>
                    <Route path=path!("login") view=Login/>
                    <ParentRoute path=path!("") view=AppLayout>
                        <Route path=path!("") view=Home/>
                    </ParentRoute>
                </Routes>
            </AuthProvider>
        </Router>
    }
}

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="flex bg-light-gray">
            <NavBar/>
            <main class="w-full rounded-md bg-white p-4 m-4 shadow-xl">
                <Outlet/>
            </main>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().unwrap();
    view! {
        <button on:click=move |_| {
            auth_context.logout.dispatch(());
        }>
            Logout
        </button>
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
