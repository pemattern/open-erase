use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::{
    login::{AuthContext, AuthProvider, Login},
    navbar::NavBar,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <AuthProvider>
            <Router>
                <AppRoutes/>
            </Router>
        </AuthProvider>
    }
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().unwrap();
    let is_valid_user = move || Some(auth_context.user.get().is_some());

    view! {
        <Routes fallback=NotFound>
            <Route path=path!("login") view=Login/>
            <ProtectedParentRoute path=path!("")
                view=AppLayout
                condition=is_valid_user
                redirect_path=|| "/login"
            >
                <Route path=path!("") view=Home/>
                <Route path=path!("reports") view=Home/>
                <Route path=path!("images") view=Home/>
                <Route path=path!("user_settings") view=Home/>
                <Route path=path!("admin_dashboard") view=Home/>
            </ProtectedParentRoute>
        </Routes>
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
        <a href="/">Go back</a>
    }
}
