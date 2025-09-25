pub mod not_found_page;

use leptos::prelude::*;

use crate::components::{
	about_component::AboutComponent, hero_component::HeroComponent,
	skills_component::SkillsComponent,
};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
	view! {
		<HeroComponent />
		<AboutComponent />
		<SkillsComponent />
	}
}
