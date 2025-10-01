use leptos::prelude::*;

use crate::{
	components::skills_component::skills::CardComponent,
	utils::svgs::frontend::{
		Css3, GraphQL, Html5, Htmx, JavaScript, LeptosIcon, Svelte, TailwindCSS, TypeScript,
		VueJS,
	},
};

#[component]
pub fn FrontendTab() -> impl IntoView {
	view! {
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <LeptosIcon />
			</div>
			<p class="text-sm">"Leptos WASM"</p>
		  </div>
	    </CardComponent>

	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <Svelte />
			</div>
			<p class="text-sm">"Svelte"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <VueJS />
			</div>
			<p class="text-sm">"Vue.js"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <TypeScript />
			</div>
			<p class="text-sm">"TypeScript"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <JavaScript />
			</div>
			<p class="text-sm">"JavaScript"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <Html5 />
			</div>
			<p class="text-sm">"HTML5"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <Css3 />
			</div>
			<p class="text-sm">"CSS3"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <TailwindCSS />
			</div>
			<p class="text-sm">"TailwindCSS"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <Htmx />
			</div>
			<p class="text-sm">"HTMx"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <GraphQL />
			</div>
			<p class="text-sm">"GraphQL"</p>
		  </div>
	    </CardComponent>
	}.into_any()
}
