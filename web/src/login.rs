use base64::{Engine, prelude::BASE64_STANDARD};
use gloo_net::http::Request;
use leptos::{ev::SubmitEvent, leptos_dom::logging::console_log, prelude::*};
use leptos_router::{NavigateOptions, components::A, hooks::use_navigate};
use open_erase_lib::schemas::{token::LoginResponse, user::GetUserResponse};

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

async fn get_user(token: &str) -> Result<GetUserResponse, gloo_net::Error> {
    Request::get("/api/user/me")
        .header("Authorization", format!("Bearer {}", token).as_str())
        .send()
        .await?
        .json::<GetUserResponse>()
        .await
}

async fn logout_backend() -> Result<(), gloo_net::Error> {
    Request::post("/api/auth/logout").send().await?;
    Ok(())
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
        navigate(
            "/",
            NavigateOptions {
                resolve: false,
                ..Default::default()
            },
        );
    };

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
                />
                <Input name="password"
                    ty="password"
                    bind=password
                    label="Password"
                />
                <div class="flex justify-end text-xs text-dark-blue hover:text-blue">
                    <A href="/">
                        "Forgot your password?"
                    </A>
                </div>
                <input class="mt-1 text-dark-gray cursor-pointer py-1 bg-blue rounded-sm hover:bg-light-blue focus:outline-2 outline-offset-2 outline-blue focus:shadow-xl"
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

    let login = Action::new_local(move |input: &(String, String)| {
        let input = input.clone();
        let navigate = use_navigate();
        async move {
            let result = get_access_token(input.0, input.1).await;
            console_log(format!("{:#?}", result).as_str());
            if let Ok(login_response) = result
                && let Ok(user_response) = get_user(&login_response.access_token).await
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

    let context = AuthContext {
        user,
        access_token,
        login,
        logout,
    };
    provide_context(context);

    view! { {children()} }
}
