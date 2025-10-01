pub mod backend;
pub mod frontend;
pub mod tools_others;

use leptos::prelude::*;

#[component]
pub fn CardComponent(children: Children) -> impl IntoView {
	view! {
		<div class="flex overflow-hidden relative flex-col h-auto rounded-lg border shadow-xl border-secondary/50">
			<div class="flex overflow-y-auto relative flex-col flex-auto justify-center items-center p-4 w-full h-auto subpixel-antialiased text-center break-words place-content-inherit align-items-inherit">
				{children()}
			</div>
		</div>
	}.into_any()
}
