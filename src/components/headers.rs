use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::{components::A, hooks::use_navigate};
use leptos_use::use_debounce_fn;

use crate::{components::svgs::InitialSVGLogoTransparent, mcp::ModalContextProvider};

#[component]
pub fn CTHeader() -> impl IntoView {
    let modal_context = ModalContextProvider::expect_context();

    let toggle_all_works = use_debounce_fn(
        move || modal_context.show_state.update(|val| *val = true),
        500.0,
    );

    let toggle_contact = use_debounce_fn(
        move || modal_context.contact_state.update(|val| *val = true),
        500.0,
    );

    view! {
        <nav class="flex items-center px-3 sm:px-8 md:px-12 lg:px-24 absolute top-0 w-full py-2.5 z-[999]">
            <A href="/" attr:class="group">
                <InitialSVGLogoTransparent class="h-12 w-24 text-neutral-500 group-hover:text-neutral-100 duration-400 ease-initial" />
            </A>

            <div class="mx-auto"></div>

            <div class="flex items-center gap-4">
                <LinkTag title="Go back Home" link="/" icon=icondata::BsArrowLeft />
                <LinkTag
                    title="all works"
                    on_click=move |_| {
                        toggle_all_works();
                    }
                />
                <LinkTag
                    title="contact"
                    on_click=move |_| {
                        toggle_contact();
                    }
                />
            </div>
        </nav>
    }
}

#[component]
pub fn FirstHeader() -> impl IntoView {
    let show_context = ModalContextProvider::expect_context().contact_state;

    let toggle_show = use_debounce_fn(move || show_context.update(|val| *val = true), 500.0);

    view! {
        <nav class="flex items-center px-3 sm:px-8 md:px-12 lg:px-24 absolute top-0 w-full py-2.5 z-[99]">
            <A href="/" attr:class="group">
                <InitialSVGLogoTransparent class="h-12 w-24 text-neutral-500 group-hover:text-neutral-100 duration-400 ease-initial" />
            </A>

            <div class="mx-auto"></div>

            <div class="flex items-center gap-4">
                <LinkTag title="case studies" link="#case_studies" />
                // <LinkTag title="experience" link="#" />
                <LinkTag
                    title="contact"
                    on_click=move |_| {
                        toggle_show();
                    }
                />
            </div>
        </nav>
    }
}

#[component]
pub fn LinkTag(
    #[prop(into, optional)] link: Signal<String>,
    #[prop(into)] title: Signal<String>,
    #[prop(into, optional)] icon: Option<icondata_core::Icon>,
    #[prop(into, optional)] on_click: Option<crate::BoxOneCallback<leptos::ev::MouseEvent>>,
) -> impl IntoView {
    let navigate = use_navigate();

    let on_click = move |e: leptos::ev::MouseEvent| {
        if !link.get_untracked().is_empty() {
            navigate(link.get_untracked().as_ref(), Default::default());
            return;
        }

        let Some(on_click) = on_click.as_ref() else {
            return;
        };

        on_click(e);
    };

    view! {
        <button
            on:click=on_click
            class="cursor-pointer relative items-center gap-2 inline-flex group py-0.5"
        >
            {icon
                .map(|icon| {
                    view! {
                        <Icon
                            icon=icon
                            attr:class="h-5 w-5 group-hover:-translate-x-2 text-stone-500 group-hover:text-stone-50 duration-200 ease-initial transition-all"
                        />
                    }
                })}
            <span class="absolute bottom-0 left-0 w-0 h-1 group-hover:w-full duration-400 transition-all ease-initial bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500"></span>
            <span class="z-10 text-lg capitalize font-black text-stone-500 group-hover:text-stone-50 duration-200 ease-initial">
                {move || title.get()}
            </span>
        </button>
    }
}
