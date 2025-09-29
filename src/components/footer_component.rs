use leptos::prelude::*;

#[component]
pub fn FooterComponent() -> AnyView {
	view! {
		<footer class="py-8 bg-base-300">
			<div class="container px-4 mx-auto text-center">
				<p class="mb-4">"© 2025 Alex Morgan. All rights reserved."</p>
				<p class="text-sm text-base-content">
					"Designed & build with" <span class="mx-0.5 text-red-500">"♥"</span>
					"using Leptos (WASM) & TailwindCSS"
				</p>
			</div>
		</footer>
	}
	.into_any()
}
