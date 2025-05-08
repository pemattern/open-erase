use leptos::prelude::*;
use leptos_router::components::A;

struct NavBarData {
    path: &'static str,
    text: &'static str,
}

impl NavBarData {
    const fn new(path: &'static str, text: &'static str) -> Self {
        Self { path, text }
    }
}

const NAVBAR_DATA: &[NavBarData] = &[
    NavBarData::new("/", "Home"),
    NavBarData::new("/reports", "Reports"),
    NavBarData::new("/images", "Images"),
];

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <ul>
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
    view! {
      <li>
        <A href=data.path>{data.text}</A>
      </li>
    }
}
