use leptos::prelude::*;
use leptos_router::components::Form;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div>
            "Login:"
            <Form method="POST" action="">
                <input />
            </Form>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub email: String,
}

#[derive(Clone)]
pub struct AuthContext {
    pub user: RwSignal<Option<User>>,
    pub login: Action<(String, String), Result<(), String>>,
    pub logout: Action<(), Result<(), String>>,
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let user = RwSignal::new(None::<User>);
    let login = Action::new(|_| async { Ok(()) });
    let logout = Action::new(|_| async { Ok(()) });

    let context = AuthContext {
        user,
        login,
        logout,
    };
    provide_context(context);

    view! { {children()} }
}
