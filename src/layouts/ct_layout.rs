use leptos::{prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_router::{
    components::Outlet,
    hooks::{use_location, use_navigate},
};

use crate::{
    components::{footers::LeadFooter, headers::CTHeader},
    layouts::main_layout::LoadingScreen,
    BoxOneCallback,
};

#[component]
pub fn CaseStudyLayout() -> impl IntoView {
    let show_loader = RwSignal::new(false);
    let for_show = use_context::<RwSignal<bool>>().expect("No setter for_show provide context");

    Effect::new(move || {
        let _ = use_location().pathname.get();
        show_loader.set(true);

        spawn_local(async {
            gloo_timers::future::sleep(std::time::Duration::from_millis(1100)).await;
        });

        show_loader.set(false);
    });

    view! {
        <Show when=move || !show_loader.get() fallback=|| view! { <LoadingScreen /> }>
            <OutletComponent />
            <AllWorksComponent for_show />
        </Show>
    }
}

#[component]
fn OutletComponent() -> impl IntoView {
    view! {
        <CTHeader />
        <Outlet />
        <LeadFooter />
    }
}

#[component]
fn AllWorksComponent(#[prop(into)] for_show: RwSignal<bool>) -> impl IntoView {
    let close_for_show = use_context::<RwSignal<bool>>().unwrap();

    let close_for_show_event = move |_| {
        if let Some(close_for_show_id) = document().get_element_by_id("close-for-show") {
            let _ = close_for_show_id.class_list().remove_1("swing-in-top-fwd");
            let _ = close_for_show_id.class_list().add_1("swing-out-top-fwd");
        }

        spawn_local(async {
            gloo_timers::future::sleep(std::time::Duration::from_millis(1100)).await;
        });
        close_for_show.update(|val| *val = false);
    };

    view! {
        <Show when=move || for_show.get() fallback=|| ()>
            <section
                id="all-works-component"
                class="fixed inset-0 z-[9999] overflow-y-auto bg-neutral-900/20 py-24"
            >
                // <!-- Fixed close button -->
                <button
                    id="close-for-show"
                    on:click=close_for_show_event
                    class="swing-in-top-fwd z-[1000] radient-pumpkin-bg fixed top-0 left-1/2 -translate-x-1/2 flex cursor-pointer items-center gap-1 px-7 py-5 text-xl font-extrabold tracking-wide text-neutral-50 capitalize"
                >
                    <Icon icon=icondata::CgClose attr:class="h-6 w-6" />
                    "close"
                </button>

                // <!-- Content wrapper -->
                <div class="space-y-24">
                    <WorkComponent
                        order=CompOrder::ToLeft
                        title="castlebyte techsolutions"
                        bg_img="/public/assets/castlebyte-techsolutions.png"
                        count="01"
                        op_state="Lead Developer - Team Project - Early 2025"
                        description="CastleByte delivers full-cycle services from intuitive UI/UX design and CRM/ERP systems to e-commerce development, mobile apps, SEO, and ongoing IT support—empowering businesses to scale in the digital age."
                        case_link="/case-studies/cebu-tours-adventures"
                        logo_img="https://castlebyte.pixl8media.com/images/logos/cts-logo-castle.png"
                        on_click=close_for_show_event
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
                        on_click=close_for_show_event
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
                        on_click=close_for_show_event
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
                        on_click=close_for_show_event
                    />
                </div>
            </section>
        </Show>
    }
}

#[derive(Debug, Clone)]
enum CompOrder {
    ToLeft,
    ToRight,
}

impl CompOrder {
    fn to_string(&self) -> String {
        match self {
            CompOrder::ToLeft => "flex-row right-0".to_string(),
            CompOrder::ToRight => "flex-row-reverse left-0".to_string(),
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
    #[prop(into, optional)] on_click: Option<BoxOneCallback<leptos::ev::MouseEvent>>,
) -> impl IntoView {
    let navigate = use_navigate();

    let on_click = move |e| {
        let left_comp_id: String = (move || format!("left-content-case-{}", count.get()))();
        let right_comp_id: String = (move || format!("right-content-case-{}", count.get()))();

        if let Some(left_comp_present_id) = document().get_element_by_id(&left_comp_id) {
            let _ = left_comp_present_id.class_list().remove_1("slide-in-left");
            let _ = left_comp_present_id.class_list().add_1("slide-out-left");
        }

        if let Some(right_comp_present_id) = document().get_element_by_id(&right_comp_id) {
            let _ = right_comp_present_id
                .class_list()
                .remove_1("slide-in-right");
            let _ = right_comp_present_id.class_list().add_1("slide-out-right");
        }

        spawn_local(async {
            gloo_timers::future::sleep(std::time::Duration::from_millis(1100)).await;
        });

        navigate(case_link.get().as_ref(), Default::default());

        let Some(on_click) = on_click.as_ref() else {
            return;
        };

        on_click(e);
    };

    view! {
        <div class=move || {
            format!(
                "mx-auto flex max-w-6xl shadow-2xl min-h-[calc(100vh-8rem)] {}",
                order.get().to_string().split(' ').nth(0).unwrap_or_default(),
            )
        }>
            // <!-- Left image section -->
            <div
                id=move || format!("left-content-case-{}", count.get())
                class="hidden md:flex slide-in-left relative h-[400px] lg:h-auto lg:w-1/2 bg-cover bg-center bg-no-repeat shadow-[0px_0px_12px_4px_rgba(23,0,0,0.44)]"
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
                        order.get().to_string().split(' ').nth(1).unwrap_or_default(),
                    )
                }>
                    <h1 class="text-[12.5em] font-black text-neutral-100 leading-none font-oswald">
                        {move || count.get()}
                    </h1>
                </div>
            </div>

            // <!-- Right content section -->
            <div
                id=move || format!("right-content-case-{}", count.get())
                class="slide-in-right relative flex flex-col w-full items-center justify-center gap-4 p-5 text-center"
                style="background: radial-gradient(ellipse farthest-side at bottom center, #BFB9A1 0%, #F0E8C9 50%, #FFF8D6 100%);"
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
                    class="group radient-pumpkin-bg relative inline-flex h-10 w-40 items-center justify-center"
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
