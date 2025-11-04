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
            <div class="flex bg-light-gray">
                <Router>
                    <Routes fallback=NotFound>
                        <Route path=path!("login") view=Login/>
                        <ParentRoute path=path!("") view=AppLayout>
                            <Route path=path!("") view=Home/>
                        </ParentRoute>
                    </Routes>
                </Router>
            </div>
        </AuthProvider>
    }
}

#[component]
pub fn AppLayout() -> impl IntoView {
    let navigate = use_navigate();
    let auth_context = use_context::<AuthContext>().unwrap();
    if auth_context.user.get().is_none() {
        navigate("/login", Default::default());
    }
    view! {

        <NavBar/>
        <main class="w-full rounded-md bg-white p-4 m-4">
            <Outlet/>
        </main>
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
