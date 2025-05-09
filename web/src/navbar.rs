use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_location};

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
        <nav>
            <ul class="max-w-64 h-screen m-4 text-dark-gray font-sans">
                {NAVBAR_DATA
                    .iter()
                    .map(|item| view! {
                        <li>
                            <NavBarEntry data=item/>
                        </li>
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
}

#[component]
fn NavBarEntry(data: &'static NavBarData) -> impl IntoView {
    let location = use_location();
    let is_current = move || location.pathname.get() == data.path;

    view! {
        <li class="m-2 text-xl">
            <div class=(["bg-light-blue", "text-dark-blue", "rounded-md", "bold"], move || is_current())>
                <div class="p-4">
                    <A href=data.path>
                        <div class="ml-2 flex items-center gap-4">
                            <Icon icon={data.icon} />
                            {data.text}
                        </div>
                    </A>
                </div>
            </div>
        </li>
    }
}
