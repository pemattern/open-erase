use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_location};

#[component]
fn Logo() -> impl IntoView {
    view! {
        <A href="/">
            <div class="flex font-mono text-4xl pb-4">
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
    NavBarData::new("/", "Home", icondata::FaHouseSolid),
    NavBarData::new("/reports", "Reports", icondata::FaClipboardSolid),
    NavBarData::new("/images", "Images", icondata::FaImageSolid),
];

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="flex-col w-72 h-screen p-4">
            <Logo/>
            {NAVBAR_DATA
                .iter()
                .map(|item| view! {
                        <NavBarEntry data=item/>
                })
                .collect_view()}
        </nav>
    }
}

#[component]
fn NavBarEntry(data: &'static NavBarData) -> impl IntoView {
    let location = use_location();
    let is_current = move || location.pathname.get() == data.path;

    view! {
        <div class=(["bg-light-blue", "text-dark-blue", "rounded-md", "bold"], move || is_current())>
            <div class="p-4 w-fit text-lg">
                <A href=data.path>
                    <div class="ml-2 flex items-center gap-4">
                        <Icon icon={data.icon} />
                        {data.text}
                    </div>
                </A>
            </div>
        </div>
    }
}
