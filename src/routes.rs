use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::{layouts::*, views::*};

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "not found".into_any()>
                <ParentRoute path=StaticSegment("") view=main_layout::MainLayout>
                    <Route path=StaticSegment("") view=IndexView />
                    <Route
                        path=(StaticSegment("case-studies"), ParamSegment("case"))
                        view=case_studies::CaseStudiesView
                    />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
