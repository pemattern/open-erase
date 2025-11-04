use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_location};

#[component]
fn Logo() -> impl IntoView {
    view! {
        <A href="/">
            <div class="flex justify-center text-2xl p-4">
                <div class="text-dark-blue">open</div>
                <div class="dark-gray">erase</div>
            </div>
        </A>
    }
}

struct NavBarData {
    groups: &'static [NavBarGroupData],
}

struct NavBarGroupData {
    header: &'static str,
    entries: &'static [NavBarEntryData],
}

struct NavBarEntryData {
    path: &'static str,
    text: &'static str,
    icon: icondata::Icon,
}

impl NavBarEntryData {
    const fn new(path: &'static str, text: &'static str, icon: icondata::Icon) -> Self {
        Self { path, text, icon }
    }
}

const NAVBAR_DATA: NavBarData = NavBarData {
    groups: &[NAVBAR_HOME_GROUP_DATA, NAVBAR_HOME_SETTINGS_DATA],
};

const NAVBAR_HOME_GROUP_DATA: NavBarGroupData = NavBarGroupData {
    header: "Home",
    entries: &[
        NavBarEntryData::new("/", "Home", icondata::OcHomeLg),
        NavBarEntryData::new("/reports", "Reports", icondata::OcRepoLg),
        NavBarEntryData::new("/images", "Images", icondata::OcImageLg),
    ],
};

const NAVBAR_HOME_SETTINGS_DATA: NavBarGroupData = NavBarGroupData {
    header: "Settings",
    entries: &[
        NavBarEntryData::new("/user_settings", "Settings", icondata::OcGearLg),
        NavBarEntryData::new("/admin_dashboard", "Dashboard", icondata::OcGlobeLg),
    ],
};

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="w-72 h-screen">
            <Logo/>
            <div class="flex flex-col gap-y-2 pl-4 pr-2">
                {NAVBAR_DATA
                    .groups
                    .iter()
                    .map(|group| view! {
                        <NavBarGroup data=group/>
                    })
                    .collect_view()}
            </div>
        </nav>
    }
}

#[component]
fn NavBarGroup(data: &'static NavBarGroupData) -> impl IntoView {
    view! {
        <div class="pl-4">
            <div class="pb-1 text-sm text-gray">{data.header}</div>
            <div class="flex flex-col gap-y-2 pr-2">
                {data.entries
                    .iter()
                    .map(|entry| view! {
                        <NavBarEntry data=entry/>
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
fn NavBarEntry(data: &'static NavBarEntryData) -> impl IntoView {
    let location = use_location();
    let is_current = move || location.pathname.get() == data.path;

    view! {
        <A href=data.path>
            <div class="border-1 rounded-md text-md outline-2 outline-offset-2 hover:bg-white hover:border-light-blue"
                class:bg-white=move || is_current()
                class:border-light-blue=move || is_current()
                class:border-transparent=move || !is_current()
                class:outline-blue=move || is_current()
                class:outline-transparent=move || !is_current()
                class:shadow-xl=move || is_current()>
                <div class="flex items-center w-fit p-2 ml-2 gap-4">
                            <Icon icon={data.icon}/>
                            {data.text}
                </div>
            </div>
        </A>
    }
}
