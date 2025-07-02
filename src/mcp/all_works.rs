use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;
use leptos_use::use_debounce_fn;

use crate::mcp::ModalContextProvider;

#[component]
pub fn AllWorksComponent() -> impl IntoView {
    let show_context = ModalContextProvider::expect_context().show_state;

    let execute_toggle = use_debounce_fn(move || show_context.update(|val| *val = false), 600.0);
    let close_for_show_event = move |_| {
        document()
            .get_element_by_id("all_works_section")
            .unwrap()
            .class_list()
            .add_1("puff-out-center")
            .unwrap();
        execute_toggle();
    };

    view! {
        <section
            id="all_works_section"
            class="fixed inset-0 z-[9999] overflow-y-auto bg-neutral-900/20 py-24"
        >
            // <!-- Fixed close button -->
            <button
                id="close-for-show"
                on:click=close_for_show_event
                class="z-[1000] swing-in-top-fwd radient-pumpkin-bg fixed top-0 left-1/2 -translate-x-1/2 flex cursor-pointer items-center gap-1 px-7 py-5 text-xl font-extrabold tracking-wide text-neutral-50 capitalize {}"
            >
                <Icon icon=icondata::CgClose attr:class="h-6 w-6" />
                "close"
            </button>

            // <!-- Content wrapper -->
            <div class="space-y-24 px-3">
                <WorkComponent
                    order=CompOrder::ToLeft
                    title="castlebyte techsolutions"
                    bg_img="/public/assets/castlebyte-techsolutions.png"
                    count="01"
                    op_state="Lead Developer - Team Project - Early 2025"
                    description="CastleByte delivers full-cycle services from intuitive UI/UX design and CRM/ERP systems to e-commerce development, mobile apps, SEO, and ongoing IT support—empowering businesses to scale in the digital age."
                    case_link="/case-studies/castlebyte-techsolutions"
                    logo_img="https://castlebyte.pixl8media.com/images/logos/cts-logo-castle.png"
                />
                <WorkComponent
                    order=CompOrder::ToRight
                    title="pixl8multimedia"
                    bg_img="/public/assets/pixl8media.jpeg"
                    count="02"
                    op_state="Lead Developer - Website Redesign - Middle 2024"
                    description="Pixl8Media functions as a platform which turns stories into genuine visual representations that outstrip their textual manifestation. Through a combination of creative direction, visual storytelling, and multimedia production, Pixl8Multimedia crafts impactful narratives that captivate audiences across digital platforms."
                    case_link="/case-studies/pixl8multimedia"
                    logo_img="https://pixl8media.com/images/logos/pixel8media-logo-edited.png"
                />
                <WorkComponent
                    order=CompOrder::ToLeft
                    title="GoldenYears Carehome"
                    bg_img="/public/assets/goldenyears.png"
                    count="03"
                    op_state="Front-end Developer - Website Design - Eearly 2025"
                    description="GoldenYears CareHome is a sanctuary of compassion dedicated to the well-being of seniors and individuals with dementia. Designed like a home, not a facility, the website focuses on family reassurance, service transparency, and easy-to-navigate booking and care details—blending empathy with accessible digital experience."
                    case_link="/case-studies/goldenyears"
                    logo_img="/public/assets/logos/goldenyears.png"
                />
                <WorkComponent
                    order=CompOrder::ToRight
                    title="cebu tours & adventures"
                    bg_img="/public/assets/cebu-tours-adventures.png"
                    count="04"
                    op_state="Lead Developer - Team Project - Early 2025"
                    description="Cebu Tours and Adventures offers curated travel experiences that highlight the cultural richness and natural wonders of Cebu. From adrenaline-packed whale shark encounters to serene island hopping and historical city tours, the platform makes booking seamless while showcasing vivid imagery, detailed itineraries, and authentic Filipino hospitality through a mobile-first design."
                    case_link="/case-studies/cebu-tours-adventures"
                    logo_img="/public/assets/logos/cebu-tours-adventures.png"
                />
            </div>
        </section>
    }
}

#[derive(Debug, Clone)]
enum CompOrder {
    ToLeft,
    ToRight,
}

impl CompOrder {
    fn as_str(&self) -> &'static str {
        match self {
            CompOrder::ToLeft => "flex-row right-0",
            CompOrder::ToRight => "flex-row-reverse left-0",
        }
    }
}

#[component]
fn WorkComponent(
    #[prop(into)] order: Signal<CompOrder>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] bg_img: Signal<String>,
    #[prop(into)] count: Signal<String>,
    #[prop(into)] op_state: Signal<String>,
    #[prop(into)] description: Signal<String>,
    #[prop(into)] case_link: Signal<String>,
    #[prop(into)] logo_img: Signal<String>,
) -> impl IntoView {
    let navigate = use_navigate();
    let right_animate_class = RwSignal::new("slide-in-right");
    let left_animate_class = RwSignal::new("slide-in-left");
    let show_context = ModalContextProvider::expect_context().show_state;

    let execute_toggle = use_debounce_fn(move || show_context.update(|val| *val = false), 600.0);

    let on_click = move |_| {
        if show_context.get() {
            right_animate_class.update(|val| *val = "slide-out-right");
            left_animate_class.update(|val| *val = "slide-out-left");
        } else {
            right_animate_class.update(|val| *val = "slide-in-right");
            left_animate_class.update(|val| *val = "slide-in-left");
        }
        execute_toggle();
        let temp_navigate = navigate.clone();
        temp_navigate(case_link.get().as_ref(), Default::default());
    };

    view! {
        <div class=move || {
            format!(
                "mx-auto flex max-w-6xl shadow-2xl h-[calc(100vh-20rem)] md:h-[calc(100vh-8rem)] {}",
                order.get().as_str().split_whitespace().next().unwrap_or_default(),
            )
        }>
            // <!-- Left image section -->
            <div
                class=move || {
                    format!(
                        "hidden md:flex relative h-[400px] lg:h-auto lg:w-1/2 bg-cover bg-center bg-no-repeat shadow-[0px_0px_12px_4px_rgba(23,0,0,0.44)] {}",
                        left_animate_class.get(),
                    )
                }
                style=move || {
                    format!(
                        "background-image: linear-gradient(to right, rgba(0,0,0,0.5), rgba(0,0,0,0.5)), url('{}');",
                        bg_img.get(),
                    )
                }
            >
                <div class=move || {
                    format!(
                        "absolute bottom-0 p-3 {}",
                        order.get().as_str().split_whitespace().nth(1).unwrap_or_default(),
                    )
                }>
                    <h1 class="text-[12.5em] font-black text-neutral-100 leading-none font-oswald">
                        {move || count.get()}
                    </h1>
                </div>
            </div>

            // <!-- Right content section -->
            <div
                class=move || {
                    format!(
                        "relative flex flex-col w-full items-center justify-center gap-4 p-5 text-center {}",
                        right_animate_class.get(),
                    )
                }
                style="background: linear-gradient(to bottom right, #f3f8ff, #deecff, #c6cfff);"
            >
                <h1
                    class="text-neutral-800 font-bold font-oswald capitalize"
                    style="margin-bottom: .1875566421em; font-size: 3.75em; line-height: 1.1253398526;"
                >
                    {move || title.get()}
                </h1>
                <h3 class="font-medium text-neutral-500 uppercase">{move || op_state.get()}</h3>
                <h2 class="text-[.9375em] text-neutral-700">{move || description.get()}</h2>

                <button
                    on:click=on_click
                    class="group radient-pumpkin-bg relative inline-flex h-10 w-40 items-center justify-center cursor-pointer"
                >
                    <span class="radient-medium-blue-bg absolute top-0 right-0 h-full w-0 transition-all duration-500 ease-in-out group-hover:right-auto group-hover:left-0 group-hover:w-full"></span>
                    <span class="z-10 text-xl font-medium tracking-wide text-slate-100">
                        "Case study"
                    </span>
                </button>

                // <!-- Footer logo -->
                <div class="absolute bottom-3 left-1/2 flex w-full -translate-x-1/2 items-center justify-center text-neutral-400">
                    <img src=move || logo_img.get() alt="logo-image" class="h-12 grayscale-100" />
                    <h1 class="text-2xl font-bold uppercase">{move || title.get()}</h1>
                </div>
            </div>
        </div>
    }
}
