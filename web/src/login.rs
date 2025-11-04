use leptos::prelude::*;
use leptos_router::components::Form;
use open_erase_lib::schemas::token::LoginResponse;

async fn get_access_token(email: String, password: String) -> Result<LoginResponse, String> {
    Err(String::from("nope"))
}

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div>
            "Login:"
            <Form method="POST" action="">
                <input />
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
    pub login: Action<(String, String), Result<LoginResponse, String>>,
    pub logout: Action<(), Result<(), String>>,
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let user = RwSignal::new(None::<User>);
    let login = Action::new(|input: &(String, String)| {
        let input = input.clone();
        async move { get_access_token(input.0, input.1).await }
    });
    let logout = Action::new(|_| async { Ok(()) });

    let context = AuthContext {
        user,
        login,
        logout,
    };
    provide_context(context);

    view! { {children()} }
}
