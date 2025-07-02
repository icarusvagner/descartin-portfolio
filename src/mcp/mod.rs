pub mod all_works;
pub mod contact_me;

use leptos::{context::Provider, prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_meta::Html;
use leptos_use::storage::use_local_storage;

#[derive(Clone, Copy)]
pub struct ModalContextProvider {
    pub contact_state: RwSignal<bool>,
    pub dark: RwSignal<bool>,
    pub show_state: RwSignal<bool>,
    pub show_menu: RwSignal<bool>,
}

impl ModalContextProvider {
    pub fn new() -> Self {
        let (stored_dark, _, _) = use_local_storage::<bool, JsonSerdeCodec>("dark");

        let dark_value = stored_dark.get();

        Self {
            contact_state: RwSignal::new(false),
            show_state: RwSignal::new(false),
            show_menu: RwSignal::new(false),
            dark: RwSignal::new(dark_value),
        }
    }

    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn update_theme(&self) {
        let (_, set_state, _) = use_local_storage::<bool, JsonSerdeCodec>("dark");

        self.dark.update(|val| *val = !*val);
        set_state.set(self.dark.get());
    }
}

#[component]
pub fn ContextProvider(children: Children) -> impl IntoView {
    let config_injection = ModalContextProvider::new();

    let theme_attr = move || {
        if config_injection.dark.get() {
            "dark"
        } else {
            "light"
        }
    };

    view! {
        <Provider value=config_injection>
            <Html {..} class="scroll-smooth" attr:data-theme=theme_attr />
            {children()}
        </Provider>
    }
}
