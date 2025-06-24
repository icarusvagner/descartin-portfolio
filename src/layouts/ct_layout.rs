use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_location};

use crate::{
    components::{footers::LeadFooter, headers::CTHeader},
    layouts::main_layout::LoadingScreen,
};

#[component]
pub fn CaseStudyLayout() -> impl IntoView {
    let show_loader = RwSignal::new(false);

    Effect::new(move || {
        let _ = use_location().pathname.get();
        show_loader.set(true);
        set_timeout(
            move || show_loader.set(false),
            std::time::Duration::from_millis(1100),
        );
    });

    view! {
        <Show when=move || !show_loader.get() fallback=|| view! { <LoadingScreen /> }>
            <OutletComponent />
        </Show>
    }
}

#[component]
fn OutletComponent() -> impl IntoView {
    view! {
        <CTHeader />
        <Outlet />
        <LeadFooter />
    }
}
