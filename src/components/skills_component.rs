mod skills;

use leptos::prelude::*;

use crate::components::skills_component::skills::{backend::BackendTab, frontend::FrontendTab};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TabType {
	Frontend,
	Backend,
	Others,
}

#[component]
pub fn SkillsComponent() -> AnyView {
	let tab_view = RwSignal::new(TabType::Frontend);

	view! {
		<section id="skills" class="container py-20 px-4 mx-auto">
			<div>
				<h2 class="mb-2 text-3xl font-bold">"Skills & Technologies"</h2>
				<div class="mb-8 w-20 h-1 bg-primary"></div>
				<div role="tablist" class="tabs tabs-border">
					<a
						role="tab"
						class=move || {
							format!(
								"tab text-primary [--tab-bg:primary] [--tab-border-color:primary] {}",
								if tab_view.get().eq(&TabType::Frontend) {
									"tab-active"
								} else {
									""
								},
							)
						}
						on:click=move |_| tab_view.set(TabType::Frontend)
					>
						"Frontend"
					</a>
					<a
						role="tab"
						class=move || {
							format!(
								"tab text-primary [--tab-border-color:primary] [--tab-bg:primary] {}",
								if tab_view.get().eq(&TabType::Backend) { "tab-active" } else { "" },
							)
						}
						on:click=move |_| tab_view.set(TabType::Backend)
					>
						"Backend"
					</a>
					<a
						role="tab"
						class=move || {
							format!(
								"tab text-primary [--tab-border-color:primary] [--tab-bg:primary] {}",
								if tab_view.get().eq(&TabType::Others) { "tab-active" } else { "" },
							)
						}
						on:click=move |_| tab_view.set(TabType::Others)
					>
						"Tools & Others"
					</a>
				</div>
				<TabView tab_type=tab_view />
			</div>
		</section>
	}
	.into_any()
}

#[component]
fn TabView(#[prop(into)] tab_type: Signal<TabType>) -> AnyView {
	view! {
		<div class="grid grid-cols-2 gap-4 mt-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
			{move || {
				match tab_type.get() {
					TabType::Frontend => view! { <FrontendTab /> }.into_any(),
					TabType::Backend => view! { <BackendTab /> }.into_any(),
					_ => view! { "Tools & Others" }.into_any(),
				}
			}}
		</div>
	}
	.into_any()
}
