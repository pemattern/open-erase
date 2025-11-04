use std::sync::LazyLock;

use leptos::prelude::*;
use open_erase_lib::schemas::token::LoginResponse;
use reqwest::Client;

static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

async fn get_access_token(
    email: String,
    password: String,
) -> Result<LoginResponse, reqwest::Error> {
    CLIENT
        .post("/api/auth/login")
        .basic_auth(email, Some(password))
        .send()
        .await?
        .json::<LoginResponse>()
        .await
}

#[component]
pub fn Login() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().unwrap();
    let email = RwSignal::new(String::default());
    let password = RwSignal::new(String::default());

    view! {
        <div>
            "Login:"
            <form
                on:submit=move |ev| {
                    ev.prevent_default();
                    auth_context.login.dispatch((email.get(), password.get()));
                }
            >
                <input type="email"
                    bind:value=email
                />
                <input type="password"
                    bind:value=password
                />
                <button type="submit">
                    Login
                </button>
            </form>
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
    pub login: Action<(String, String), Result<LoginResponse, reqwest::Error>>,
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
