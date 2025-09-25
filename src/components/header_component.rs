use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

use crate::{mcp::ModalContextProvider, utils::svgs::InitialSVGLogo};

#[component]
pub fn HeaderComponent() -> AnyView {
	let context = ModalContextProvider::expect_context();

	view! {
		<nav class="sticky inset-0 py-4 px-4 w-full border-b border-base-content z-[99] font-poppins bg-base-200/90 backdrop-brightness-150 backdrop-blur-md">
			<div class="container flex justify-between items-center mx-auto">
				<A href="/" attr:class="cursor-pointer group">
					<InitialSVGLogo class="h-12" />
				</A>

				<div class="flex gap-5 items-center text-base-content">
					<LinkTag title="About" link="#about" />
					<LinkTag title="Skills" link="#skills" />
					<LinkTag title="Projects" link="#projects" />
					<LinkTag title="Experience" link="#experience" />
					<LinkTag title="Contact" link="#contact" />
				</div>

				<div class="flex gap-5 items-center">
					<A href="" attr:class="btn btn-soft btn-info">
						"Get in touch"
					</A>
					<button
						class="btn btn-ghost"
						on:click=move |_| {
							context.update_theme();
						}
					>
						<Icon icon=icondata::WiMoonAltFirstQuarter attr:class="h-8 w-8" />
					</button>
				</div>
			</div>
		</nav>
	}
	.into_any()
}

#[component]
fn LinkTag(
	#[prop(into)] title: MaybeProp<String>,
	#[prop(into)] link: MaybeProp<String>,
) -> AnyView {
	view! {
		<A
			href=move || link.get().unwrap_or_default()
			attr:class="relative px-3 py-2 overflow-hidden text-center hover:text-primary duration-200 ease-in-out transition-all"
		>
			{move || title.get().unwrap_or_default()}
		</A>
	}
	.into_any()
}
