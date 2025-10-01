use leptos::prelude::*;

use crate::{
	components::skills_component::skills::CardComponent,
	utils::svgs::backend::{
		Docker, DuckDB, ExpressJS, Firebase, MongoDB, MySQL, NodeJS, PostgreSQL, Redis,
		RustLang,
	},
};

#[component]
pub fn BackendTab() -> impl IntoView {
	view! {
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <PostgreSQL />
			</div>
			<p class="text-sm">"PostgreSQL"</p>
		  </div>
	    </CardComponent>

	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <RustLang />
			</div>
			<p class="text-sm">"Rust Lang"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <MySQL />
			</div>
			<p class="text-sm">"MySQL/MariaDB"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <MongoDB />
			</div>
			<p class="text-sm">"MongoDB"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <NodeJS />
			</div>
			<p class="text-sm">"NodeJS"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <ExpressJS />
			</div>
			<p class="text-sm">"ExpressJS"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <Docker />
			</div>
			<p class="text-sm">"Docker"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <DuckDB />
			</div>
			<p class="text-sm">"DuckDB"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <Redis />
			</div>
			<p class="text-sm">"Redis"</p>
		  </div>
	    </CardComponent>
	    <CardComponent>
		  <div>
			<div class="flex justify-center mb-2">
			    <Firebase />
			</div>
			<p class="text-sm">"Firebase"</p>
		  </div>
	    </CardComponent>
	}.into_any()
}
