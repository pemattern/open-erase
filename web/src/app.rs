use leptos::prelude::*;
use leptos_router::{components::*, hooks::use_navigate, path};

use crate::{
    login::{AuthContext, AuthProvider, Login},
    navbar::NavBar,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <AuthProvider>
            <Router>
                <Routes fallback=NotFound>
                    <Route path=path!("login") view=Login/>
                    <ParentRoute path=path!("") view=AppLayout>
                        <Route path=path!("") view=Home/>
                    </ParentRoute>
                </Routes>
            </Router>
        </AuthProvider>
    }
}

#[component]
pub fn AppLayout() -> impl IntoView {
    let navigate = use_navigate();
    let auth_context = use_context::<AuthContext>().unwrap();

    Effect::new(move |_| {
        if auth_context.user.get().is_none() {
            navigate("/login", Default::default());
        }
    });

    view! {
        <div class="flex bg-light-gray">
            <NavBar/>
            <main class="w-full rounded-md bg-white p-4 m-4">
                <Outlet/>
            </main>
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
