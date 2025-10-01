use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
	components::{ParentRoute, Route, Router, Routes},
	StaticSegment,
};

use crate::{
	layouts::main_layout::MainLayout,
	mcp::ContextProvider,
	pages::{not_found_page::NotFoundPage, HomePage},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
	    <!DOCTYPE html>
	    <html lang="en">
		  <head>
			<meta charset="utf-8" />
			<meta name="viewport" content="width=device-width, initial-scale=1" />
			<AutoReload options=options.clone() />
			<HydrationScripts options />
			<MetaTags />
		  </head>
		  <body class="h-full min-h-screen">
			<App />
		  </body>
	    </html>
	}
	.into_any()
}

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
	    // injects a stylesheet into the document <head>
	    // id=leptos means cargo-leptos will hot-reload this stylesheet
	    <Stylesheet id="leptos" href="/pkg/portfolio-v1.css" />

	    // sets the document title
	    <Title text="Lance Phillip Descartin - Interactive Frontend" />

	    // content for this welcome page
	    <ContextProvider>
		  <Router>
			<Routes fallback=|| view! { <NotFoundPage /> }>
			    <ParentRoute path=StaticSegment("") view=MainLayout>
				  <Route path=StaticSegment("") view=HomePage />
			    </ParentRoute>
			</Routes>
		  </Router>
	    </ContextProvider>
	}
	.into_any()
}
