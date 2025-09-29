use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::{footer_component::FooterComponent, header_component::HeaderComponent};

#[component]
pub fn MainLayout() -> AnyView {
	view! {
		<main class="bg-base-200 text-base-content">
			<HeaderComponent />
			<div class="pt-14 w-full h-full">
				<Outlet />
			</div>
			<FooterComponent />
		</main>
	}
	.into_any()
}
