pub mod all_works;
pub mod contact_me;

use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub struct ModalContextProvider {
    pub contact_state: RwSignal<bool>,
    pub show_state: RwSignal<bool>,
}

impl ModalContextProvider {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[component]
pub fn ContextProvider(children: Children) -> impl IntoView {
    let show_state = RwSignal::new(false);
    let contact_state = RwSignal::new(false);

    let config_injection = ModalContextProvider {
        contact_state,
        show_state,
    };

    view! { <Provider value=config_injection>{children()}</Provider> }
}
