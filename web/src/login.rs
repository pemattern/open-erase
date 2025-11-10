use base64::{Engine, prelude::BASE64_STANDARD};
use gloo_net::http::Request;
use leptos::{ev::SubmitEvent, prelude::*};
use leptos_router::{NavigateOptions, hooks::use_navigate};
use open_erase_lib::schemas::{
    token::{LoginResponse, RefreshResponse},
    user::GetUserResponse,
};

use crate::{input::Input, navbar::Logo};

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

async fn get_me(token: &str) -> Result<GetUserResponse, gloo_net::Error> {
    Request::get("/api/users/me")
        .header("Authorization", format!("Bearer {}", token).as_str())
        .send()
        .await?
        .json::<GetUserResponse>()
        .await
}

async fn refresh() -> Result<RefreshResponse, gloo_net::Error> {
    Request::post("/api/auth/refresh")
        .send()
        .await?
        .json::<RefreshResponse>()
        .await
}

async fn logout_backend() -> Result<(), gloo_net::Error> {
    Request::post("/api/auth/logout").send().await?;
    Ok(())
}

#[component]
pub fn Login() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().unwrap();

    let email = RwSignal::new(String::default());
    let password = RwSignal::new(String::default());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        auth_context.login.dispatch((email.get(), password.get()));
    };

    Effect::new(move || {
        auth_context.refresh.dispatch("/".to_string());
    });

    view! {
        <div class="flex items-center justify-center h-screen bg-light-gray">
            <form class="flex flex-col p-4 rounded-lg gap-y-1 bg-white shadow-xl"
                on:submit=on_submit
            >
                <Logo/>
                <Input name="email"
                    ty="text"
                    bind=email
                    label="Email"
                    autofocus=true
                />
                <Input name="password"
                    ty="password"
                    bind=password
                    label="Password"
                />
                <div class="flex justify-end text-xs text-dark-blue hover:underline">
                    <a href="/" class="rounded-sm">
                        "Forgot your password?"
                    </a>
                </div>
                <input class="mt-1 text-dark-gray cursor-pointer py-1 bg-blue rounded-sm hover:bg-light-blue"
                    type="submit"
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
    pub access_token: RwSignal<Option<String>>,
    pub login: Action<(String, String), ()>,
    pub refresh: Action<String, ()>,
    pub logout: Action<(), ()>,
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let user = RwSignal::new(None::<User>);
    let access_token = RwSignal::new(None::<String>);

    let reset = move || {
        user.set(None);
        access_token.set(None);
    };

    let login = Action::new_local(move |input: &(String, String)| {
        let (email, password) = input.clone();
        let navigate = use_navigate();
        async move {
            if let Ok(login_response) = get_access_token(email, password).await
                && let Ok(user_response) = get_me(&login_response.access_token).await
            {
                access_token.set(Some(login_response.access_token));
                user.set(Some(User {
                    email: user_response.email,
                }));
                navigate(
                    "/",
                    NavigateOptions {
                        resolve: false,
                        ..Default::default()
                    },
                );
            } else {
                reset();
            }
        }
    });

    let refresh = Action::new_local(move |input: &String| {
        let navigate = use_navigate();
        let input = input.clone();
        async move {
            if let Ok(refresh_response) = refresh().await
                && let Ok(user_response) = get_me(&refresh_response.access_token).await
            {
                user.set(Some(User {
                    email: user_response.email,
                }));
                access_token.set(Some(refresh_response.access_token));
                navigate(
                    input.as_str(),
                    NavigateOptions {
                        resolve: false,
                        ..Default::default()
                    },
                );
            }
        }
    });

    let logout = Action::new_local(move |_| {
        let navigate = use_navigate();
        async move {
            let _ = logout_backend().await;
            reset();
            navigate(
                "/login",
                NavigateOptions {
                    resolve: false,
                    ..Default::default()
                },
            );
        }
    });

    let context = AuthContext {
        user,
        access_token,
        login,
        refresh,
        logout,
    };
    provide_context(context);

    view! { {children()} }
}
