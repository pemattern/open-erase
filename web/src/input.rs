use leptos::prelude::*;
use leptos::{IntoView, component, view};

#[component]
pub fn Input(
    bind: RwSignal<String>,
    name: &'static str,
    ty: &'static str,
    #[prop(optional)] label: &'static str,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <label for=name
                class="text-dark-gray font-bold text-sm"
            >
                {label}
            </label>
            <input
                name=name
                type=ty
                placeholder=placeholder
                bind:value=bind
                class="mt-1 px-2 py-1 border border-gray focus:shadow-xl focus:outline-2 outline-offset-2 outline-blue rounded-sm"
            />
        </div>
    }
}
