use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::use_debounce_fn;

use crate::mcp::ModalContextProvider;

#[component]
pub fn ContactMeSection() -> impl IntoView {
    let right_contact_section = RwSignal::new("slide-in-bottom");
    let left_contact_section = RwSignal::new("slide-in-top");
    let contact_context = ModalContextProvider::expect_context().contact_state;

    let execute_toggle = use_debounce_fn(move || contact_context.update(|val| *val = false), 600.0);

    let on_click = move |_| {
        if contact_context.get() {
            right_contact_section.update(|val| *val = "slide-out-top");
            left_contact_section.update(|val| *val = "slide-out-bottom");
        } else {
            right_contact_section.update(|val| *val = "slide-in-bottom");
            left_contact_section.update(|val| *val = "slide-in-top");
        }
        execute_toggle();
    };

    view! {
        // <!-- About Me Section -->
        <section class="fixed inset-0 z-[9999] overflow-y-auto min-h-screen">
            <div class="absolute top-1/2 left-1/2 grid h-full md:h-[calc(100vh-5rem)] w-full md:w-7xl -translate-x-1/2 -translate-y-1/2 md:grid-cols-2 shadow-2xl">
                // Left Content
                <div class=move || {
                    format!(
                        "relative bg-stone-100 p-14 flex items-center md:order-1 order-2 {}",
                        left_contact_section.get(),
                    )
                }>
                    <div
                        class="absolute top-0 right-0 z-0 flex flex-col text-right text-9xl font-black text-neutral-200"
                        style="line-height: 1.3260254597"
                    >
                        <h1>"About"</h1>
                        <h1>"Me"</h1>
                    </div>
                    <div class="relative z-20 flex flex-col justify-center gap-0.5">
                        <h1 class="contact-about">"About Me."</h1>
                        <h4 class="text-[.75em] xl:text-[.987em] font-normal text-neutral-600">
                            "Full-Stack Software Developer"
                        </h4>
                        <p class="my-6 text-[.875em] xl:text-[.99875em] leading-relaxed text-neutral-700">
                            "I'm "<span class="font-semibold">"Lance Phillip Descartin"</span>
                            ", a 25-year-old Filipino "
                            <span class="text-pumpkin">
                                "Freelancer and Full-Stack Software Developer. "
                            </span>
                            "I specialize in building clean, scalable, and user-focused digital solutions from intuitive frontend interfaces to high-performance backend systems. With every line of code, I aim to bring ideas to life while delivering "
                            <span class="text-pumpkin">"real-world impact and value."</span>
                        </p>
                        <blockquote class="text-md xl:text-lg mb-6 border-l-4 border-indigo-500 pl-4 text-gray-500 italic">
                            r#""Whatever you do, work at it with all your heart, as working for the Lord, not for human masters.""#
                            <br />
                            <span class="mt-2 inline-block text-sm font-medium text-indigo-700">
                                "— Colossians 3:23 (NIV)"
                            </span>
                        </blockquote>

                        // <!-- Tech Stack Buttons -->
                        <div class="mt-4 grid grid-cols-2 md:grid-cols-4 gap-4">
                            // <!-- Frontend -->
                            <button
                                type="button"
                                class="inline-flex items-center gap-2 rounded-full border border-indigo-600 px-4 py-2 text-sm font-medium text-indigo-600 transition hover:bg-indigo-50"
                                data-modal-target="frontendModal"
                            >
                                <Icon
                                    icon=icondata::ImHtmlFive
                                    attr:class="h-5 w-5 text-indigo-600"
                                />
                                "Frontend"
                            </button>

                            // <!-- Backend -->
                            <button
                                type="button"
                                class="inline-flex items-center gap-2 rounded-full border border-rose-600 px-4 py-2 text-sm font-medium text-rose-600 transition hover:bg-rose-50"
                                data-modal-target="backendModal"
                            >
                                <Icon
                                    icon=icondata::ImDatabase
                                    attr:class="h-5 w-5 text-rose-600"
                                />
                                "Backend"
                            </button>

                            // <!-- DevOps -->
                            <button
                                type="button"
                                class="inline-flex items-center gap-2 rounded-full border border-teal-600 px-4 py-2 text-sm font-medium text-teal-600 transition hover:bg-teal-50"
                                data-modal-target="devopsModal"
                            >
                                <Icon
                                    icon=icondata::FaConnectdevelopBrands
                                    attr:class="h-5 w-5 text-teal-600"
                                />
                                "DevOps"
                            </button>

                            // <!-- Others -->
                            <button
                                type="button"
                                class="inline-flex items-center gap-2 rounded-full border border-gray-600 px-4 py-2 text-sm font-medium text-gray-700 transition hover:bg-gray-100"
                                data-modal-target="othersModal"
                            >
                                <Icon
                                    icon=icondata::RiBox3OthersFill
                                    attr:class="h-5 w-5 text-gray-600"
                                />
                                "Others"
                            </button>
                        </div>
                    </div>
                </div>

                // Right content
                <div class=move || {
                    format!(
                        "flex flex-col gap-0.5 bg-stone-800 p-18 md:order-2 order-1 justify-center {}",
                        right_contact_section.get(),
                    )
                }>
                    <div class="flex items-center justify-between">
                        <h1 class="text-2xl xl:text-4xl font-bold text-stone-100">"Let's talk."</h1>
                        <button
                            on:click=on_click
                            class="group relative inline-flex h-10 w-10 cursor-pointer"
                        >
                            // <!-- Bar 1 -->
                            <div class="absolute top-1/2 left-1/2 h-[3px] w-10 origin-center -translate-x-1/2 -translate-y-1/2 rotate-45 bg-stone-100 transition-all duration-300 ease-in-out group-hover:rotate-[-135deg] group-hover:linear-pumpkin group-hover:bg-pumpkin"></div>
                            // <!-- Bar 2 -->
                            <div class="absolute top-1/2 left-1/2 h-[3px] w-10 origin-center -translate-x-1/2 -translate-y-1/2 -rotate-45 bg-stone-100 transition-all duration-300 ease-in-out group-hover:rotate-[135deg] group-hover:linear-pumpkin group-hover:bg-pumpkin"></div>
                        </button>
                    </div>
                    <p class="mb-8 text-xs xl:text-xl text-gray-600 dark:text-gray-300">
                        "New project, freelance inquiry, collaboration, or even a coffee."
                    </p>

                    <div class="mt-5 flex flex-col gap-5 mb-6">
                        <label for="fullname" class="relative">
                            <input
                                id="fullname"
                                required
                                type="text"
                                class="peer cursor-text w-full border border-stone-500 px-5 py-2.5 text-stone-100 transition-all duration-300 ease-in-out outline-none focus:border-sky-700"
                            />
                            <span class="absolute pointer-events-none top-2.5 left-3 ml-1 bg-stone-800 px-1.5 text-base text-stone-500 transition-all duration-300 ease-in-out peer-valid:-translate-y-5 peer-valid:text-sm peer-valid:text-stone-500 peer-focus-within:-translate-y-5 peer-focus-within:text-sm peer-focus-within:text-sky-700">
                                "Fullname *"
                            </span>
                        </label>

                        <label for="email" class="relative">
                            <input
                                id="email"
                                required
                                type="email"
                                class="peer cursor-text w-full border border-stone-500 px-5 py-2.5 text-stone-100 transition-all duration-300 ease-in-out outline-none focus:border-sky-700"
                            />
                            <span class="absolute pointer-events-none top-2.5 left-3 ml-1 bg-stone-800 px-1.5 text-base text-stone-500 transition-all duration-300 ease-in-out peer-valid:-translate-y-5 peer-valid:text-sm peer-valid:text-stone-500 peer-focus-within:-translate-y-5 peer-focus-within:text-sm peer-focus-within:text-sky-700">
                                "Email Address *"
                            </span>
                        </label>

                        <label for="fullname" class="relative">
                            <textarea
                                required
                                name="message"
                                id="message"
                                cols="30"
                                rows="4"
                                class="peer cursor-text w-full border border-stone-500 px-5 py-2.5 text-stone-100 transition-all duration-300 ease-in-out outline-none focus:border-sky-700"
                            ></textarea>
                            <span class="absolute pointer-events-none top-3 left-3 ml-1 bg-stone-800 px-1.5 text-base text-stone-500 transition-all duration-300 ease-in-out peer-valid:-translate-y-3 peer-valid:text-sm peer-valid:text-stone-500 peer-focus-within:-translate-y-6 peer-focus-within:text-sm peer-focus-within:text-sky-700">
                                "Message *"
                            </span>
                        </label>
                    </div>

                    <button class="flex cursor-pointer self-start w-44 h-12 text-center radient-pumpkin-bg text-stone-100 items-center justify-center">
                        "Send Message"
                    </button>
                </div>
            </div>
        </section>
    }
}
