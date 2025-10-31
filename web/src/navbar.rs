use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_location};

#[component]
fn Logo() -> impl IntoView {
    view! {
        <A href="/">
            <div class="flex text-2xl p-4">
                <div class="text-dark-blue">open</div>
                <div class="dark-gray">erase</div>
            </div>
        </A>
    }
}

struct NavBarData {
    path: &'static str,
    text: &'static str,
    icon: icondata::Icon,
}

impl NavBarData {
    const fn new(path: &'static str, text: &'static str, icon: icondata::Icon) -> Self {
        Self { path, text, icon }
    }
}

const NAVBAR_DATA: &[NavBarData] = &[
    NavBarData::new("/", "Home", icondata::OcHomeLg),
    NavBarData::new("/reports", "Reports", icondata::OcRepoLg),
    NavBarData::new("/images", "Images", icondata::OcImageLg),
];

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="w-72 h-screen">
            <Logo/>
            <div class="flex flex-col gap-y-2 pl-4 pr-2">
                {NAVBAR_DATA
                    .iter()
                    .map(|item| view! {
                            <NavBarEntry data=item/>
                    })
                    .collect_view()}
            </div>
        </nav>
    }
}

#[component]
fn NavBarEntry(data: &'static NavBarData) -> impl IntoView {
    let location = use_location();
    let is_current = move || location.pathname.get() == data.path;

    view! {
        <div class="border-1 rounded-md text-md outline-2 outline-offset-2"
            class:bg-white=move || is_current()
            class:border-light-blue=move || is_current()
            class:border-transparent=move || !is_current()
            class:outline-blue=move || is_current()
            class:outline-transparent=move || !is_current()
            class:shadow-xl=move || is_current()>
            <div class="p-2 w-fit">
                <A href=data.path>
                    <div class="ml-2 flex items-center gap-4">
                        <Icon icon={data.icon}/>
                        {data.text}
                    </div>
                </A>
            </div>
        </div>
    }
}
