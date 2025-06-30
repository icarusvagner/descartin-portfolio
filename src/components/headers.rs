use leptos::{either::Either, prelude::*};
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
                <InitialSVGLogoTransparent class="h-12 w-24 text-neutral-500 group-hover:dark:text-neutral-100 group-hover:text-neutral-700 duration-400 ease-initial" />
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
    let context = ModalContextProvider::expect_context();

    let toggle_show = use_debounce_fn(
        move || context.contact_state.update(|val| *val = true),
        500.0,
    );

    view! {
        <nav class="flex items-center px-3 sm:px-8 md:px-12 lg:px-24 absolute top-0 w-full py-2.5 z-[99]">
            <A href="/" attr:class="group">
                <InitialSVGLogoTransparent class="h-12 w-24 text-neutral-500 group-hover:dark:text-neutral-100 group-hover:text-neutral-700 duration-400 ease-initial" />
            </A>

            <div class="mx-auto"></div>

            <div class="flex items-center gap-4">
                <LinkTag title="case studies" link="#case_studies" />
                <LinkTag
                    title="contact"
                    on_click=move |_| {
                        toggle_show();
                    }
                />
                <LinkTag
                    icon=icondata::VsColorMode
                    on_click=move |_| {
                        context.update_theme();
                    }
                />
            </div>
        </nav>
    }
}

#[component]
pub fn LinkTag(
    #[prop(into, optional)] link: Signal<String>,
    #[prop(into, optional)] title: MaybeProp<String>,
    #[prop(into, optional)] icon: MaybeProp<icondata_core::Icon>,
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
            class="cursor-pointer relative items-center gap-2 inline-flex group py-0.5 justify-center"
        >
            {move || {
                if let Some(icon) = icon.get_untracked() {
                    Either::Left(
                        view! {
                            <Icon
                                icon=icon
                                attr:class=move || {
                                    format!(
                                        "h-5 w-5 text-stone-500 group-hover:dark:text-stone-50 group-hover:text-stone-700 duration-200 ease-initial transition-all {}",
                                        if title.get_untracked().is_some() {
                                            "group-hover:-translate-x-2"
                                        } else {
                                            "flex self-center my-0.5"
                                        },
                                    )
                                }
                            />
                        },
                    )
                } else {
                    Either::Right(())
                }
            }}
            <span class="absolute bottom-0 left-0 w-0 h-1 group-hover:w-full duration-400 transition-all ease-initial bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500"></span>
            {move || {
                if let Some(title) = title.get_untracked() {
                    Either::Left(
                        view! {
                            <span class="z-10 text-lg capitalize font-black text-stone-500 group-hover:dark:text-stone-50 group-hover:text-stone-700 duration-200 ease-initial">
                                {title}
                            </span>
                        },
                    )
                } else {
                    Either::Right(())
                }
            }}
        </button>
    }
}
