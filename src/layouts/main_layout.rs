use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::headers::FirstHeader;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <FirstHeader />
        <Outlet />
    }
}
