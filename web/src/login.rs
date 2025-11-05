use base64::{Engine, prelude::BASE64_STANDARD};
use gloo_net::http::Request;
use leptos::{ev::SubmitEvent, leptos_dom::logging::console_log, prelude::*};
use leptos_router::hooks::use_navigate;
use open_erase_lib::schemas::{token::LoginResponse, user::GetUserResponse};

async fn get_access_token(
    email: String,
    password: String,
) -> Result<LoginResponse, gloo_net::Error> {
    let encoded_credentials = BASE64_STANDARD.encode(format!("{}:{}", email, password));
    Request::post("/api/auth/login")
        .header(
            "Authorization",
            format!("Basic {}", encoded_credentials).as_str(),
        )
        .send()
        .await?
        .json::<LoginResponse>()
        .await
}

async fn get_user(token: &str) -> Result<GetUserResponse, gloo_net::Error> {
    Request::get("/api/user/me")
        .header("Authorization", format!("Bearer {}", token).as_str())
        .send()
        .await?
        .json::<GetUserResponse>()
        .await
}

#[component]
pub fn Login() -> impl IntoView {
    let navigate = use_navigate();
    let auth_context = use_context::<AuthContext>().unwrap();
    let email = RwSignal::new(String::default());
    let password = RwSignal::new(String::default());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        auth_context.login.dispatch((email.get(), password.get()));
        navigate("", Default::default());
    };

    view! {
        <div>
            "Login:"
            <form
                on:submit=on_submit
            >
                <input type="text"
                    bind:value=email
                />
                <input type="password"
                    bind:value=password
                />
                <br />
                <input type="submit"
                    value="Submit"
                />
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
    pub login: Action<(String, String), Option<User>>,
    pub logout: Action<(), Result<(), String>>,
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let user = RwSignal::new(None::<User>);
    let access_token = RwSignal::new(None::<String>);
    let login = Action::new_local(move |input: &(String, String)| {
        let input = input.clone();
        async move {
            let result = get_access_token(input.0, input.1).await;
            console_log(format!("{:#?}", result).as_str());
            if let Ok(login_response) = result
                && let Ok(user) = get_user(&login_response.access_token).await
            {
                console_log("success");
                access_token.set(Some(login_response.access_token));
                Some(User { email: user.email })
            } else {
                console_log("error");
                access_token.set(None);
                None
            }
        }
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
