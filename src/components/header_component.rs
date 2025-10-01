use leptos::prelude::*;
use leptos_icons::Icon as LeptosIcon;
use leptos_router::components::A;
use phosphor_leptos::{Icon as PhosphorIcon, LIST};

use crate::{mcp::ModalContextProvider, utils::svgs::InitialSVGLogo};

#[component]
pub fn HeaderComponent() -> impl IntoView {
	let context = ModalContextProvider::expect_context();

	view! {
		<nav class="sticky inset-0 py-4 px-4 w-full border-b border-base-content z-[99] font-poppins bg-base-200/90 backdrop-brightness-150 backdrop-blur-md">
			<div class="container flex justify-between items-center mx-auto">
				<A href="/" attr:class="cursor-pointer group">
					<InitialSVGLogo class="h-12" />
				</A>

				<div class="hidden gap-5 items-center md:flex text-base-content">
					<LinkTag title="About" link="#about" />
					<LinkTag title="Skills" link="#skills" />
					// <LinkTag title="Projects" link="#projects" />
					<LinkTag title="Experience" link="#experience" />
					<LinkTag title="Contact" link="#contact" />
				</div>

				<div class="flex gap-5 items-center">
					<A href="#contact" attr:class="btn btn-soft btn-info">
						"Get in touch"
					</A>
					// <button class="block md:hidden btn btn-ghost">
					// <PhosphorIcon icon=LIST attr:class="h-8 w-8" />
					// </button>
					<Sidebar>
						<label for="my-drawer-4" class="drawer-button btn btn-ghost">
							<PhosphorIcon icon=LIST attr:class="h-8 w-8" />
						</label>
					</Sidebar>
					<button
						class="hidden md:block btn btn-ghost"
						on:click=move |_| {
							context.update_theme();
						}
					>
						<LeptosIcon
							icon=icondata::WiMoonAltFirstQuarter
							attr:class="h-8 w-8"
						/>
					</button>
				</div>
			</div>
		</nav>
	}.into_any()
}

#[component]
fn Sidebar(children: Children) -> impl IntoView {
	view! {
	    <div class="md:hidden drawer drawer-end">
		  <input id="my-drawer-4" type="checkbox" class="drawer-toggle" />
		  <div class="drawer-content">{children()}</div>
		  <div class="drawer-side">
			<label
			    for="my-drawer-4"
			    aria-label="close sidebar"
			    class="drawer-overlay"
			></label>
			<div class="p-4 w-80 min-h-full menu bg-base-200">
			    <ul class="text-base-content">
				  <li>
					<LinkTag title="About" link="#about" />
				  </li>
				  // <li>
				  // <LinkTag title="Projects" link="#projects" />
				  // </li>
				  <li>
					<LinkTag title="Skills" link="#skills" />
				  </li>
				  <li>
					<LinkTag title="Experience" link="#experience" />
				  </li>
				  <li>
					<LinkTag title="Contact" link="#contact" />
				  </li>
				  <li>
					<LinkTag
					    title="Download Resume"
					    link="/lance-phillip-resume.docx"
					    attr:download
					/>
				  </li>
			    </ul>

			    <div class="flex gap-4 mx-auto mt-auto">
				  <A
					href="https://github.com/icarusvagner"
					attr:class="cursor-pointer"
				  >
					<LeptosIcon
					    icon=icondata::FaSquareGithubBrands
					    attr:class="h-10 w-10"
					/>
				  </A>
				  <A
					href="https://www.linkedin.com/in/lance-phillip-descartin-189139241/"
					attr:class="cursor-pointer"
				  >
					<LeptosIcon
					    icon=icondata::FaLinkedinBrands
					    attr:class="h-10 w-10"
					/>
				  </A>
				  <A
					href="https://youtube.com/@cnalecoding"
					attr:class="cursor-pointer"
				  >
					<LeptosIcon
					    icon=icondata::FaSquareYoutubeBrands
					    attr:class="h-10 w-10"
					/>
				  </A>
			    </div>
			</div>
		  </div>
	    </div>
	}
	.into_any()
}

#[component]
fn LinkTag(
	#[prop(into)] title: MaybeProp<String>,
	#[prop(into)] link: MaybeProp<String>,
) -> impl IntoView {
	view! {
		<A
			href=move || link.get().unwrap_or_default()
			attr:class="relative px-3 py-2 overflow-hidden text-center hover:text-primary duration-200 ease-in-out transition-all"
		>
			{move || title.get().unwrap_or_default()}
		</A>
	}.into_any()
}
