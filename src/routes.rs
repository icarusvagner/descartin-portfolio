use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};

use crate::{layouts::*, views::*};

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "not found".into_any()>
                <ParentRoute path=StaticSegment("") view=main_layout::MainLayout>
                    <Route path=StaticSegment("") view=IndexView />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
