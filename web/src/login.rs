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
