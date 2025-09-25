use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> AnyView {
	view! {
		<section class="flex flex-col gap-5 justify-center items-center min-h-screen">
			<div class="flex flex-col gap-5 justify-center items-center w-52">
				<h1 class="text-2xl font-semibold text-center font-poppins">
					"Oopss, Looks like that the page you are looking not found"
				</h1>
				<A href="/" attr:class="btn btn-ghost btn-md cursor-pointer text-lg">
					"Go Home"
				</A>
			</div>
		</section>
	}
	.into_any()
}
