use leptos::prelude::*;

use crate::components::NextWorkButton;

#[component]
pub fn CebuToursAdventuresView() -> impl IntoView {
    view! {
        <section
            class="min-h-[580px] xl:min-h-[53rem] bg-center bg-cover bg-no-repeat flex flex-col items-center justify-center gap-5"
            style="background-image: linear-gradient(to top, rgba(0,0,0,0.9) 0%, rgba(0,0,0,0.5) 100%), url('/public/assets/cebu-tours-adventures.png')"
        >
            <div class="flex flex-col gap-1 flex-1 items-center justify-center">
                <h1 class="text-[3.9975843648em] font-extrabold text-neutral-100 capitalize text-center">
                    "Cebu Tours & Adventure"
                </h1>
                <h3 class="text-[1.125em] italic text-neutral-100 font-normal max-w-xl text-center">
                    "Experience the beauty of life, one unforgettable adventure at a time in Cebu."
                </h3>
            </div>

            <div class="w-full px-4 py-8">
                <div class="flex flex-wrap justify-center items-center gap-x-10 gap-y-4 font-bold tracking-[2px] uppercase text-white text-center">

                    <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                        <h1 class="text-sky-400">"role"</h1>
                        <h3 class="text-neutral-100">"full-stack developer"</h3>
                    </div>
                    <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                        <h1 class="text-sky-400">"context"</h1>
                        <h3 class="text-neutral-100">"website design"</h3>
                    </div>
                    <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                        <h1 class="text-sky-400">"period"</h1>
                        <h3 class="text-neutral-100">"early 2025"</h3>
                    </div>
                </div>
            </div>
        </section>

        <section class="mb-24 min-h-screen">

            <section class="relative flex py-24 w-full flex-col items-center justify-center bg-[#0094d9]">
                <div class="flex max-w-xl flex-col items-center justify-center gap-8">
                    <h1 class="text-7xl font-black text-stone-100 capitalize">"the project"</h1>
                    <p class="text-center text-stone-200">
                        "Cebu Tours & Adventure is a Cebu-based travel company dedicated to providing unforgettable experiences across the island. Their mission is to help travelers explore the rich culture, natural beauty, and hidden gems of Cebu through thoughtfully curated tours and services."
                    </p>
                    <a
                        href="https://cebutouradventures.com"
                        class="text-stone-100 border py-2 px-5 hover:bg-stone-100 hover:text-stone-800 duration-300 transition-all ease-in-out hover:border-stone-100 text-xl font-medium"
                    >
                        "Visit Website"
                    </a>
                </div>

                <div class="absolute bottom-0 left-10 z-0">
                    <h1 class="text-4xl lg:text-7xl font-montserrat font-black tracking-widest text-stone-100/30">
                        "Cebu Tours & Adventures"
                    </h1>
                </div>
            </section>

            <section class="relative overflow-hidden">
                <div class="mx-auto grid h-full gap-12 px-3 py-12 lg:max-w-7xl lg:grid-cols-2 lg:px-0 xl:max-w-6xl">
                    <div class="flex flex-col gap-2">
                        <h1 class="case-project-title">"cebu tours & adventures"</h1>
                        <p class="mt-10 text-left text-lg font-normal tracking-wide text-neutral-600 dark:text-neutral-200">
                            "As the " <span class="text-pumpkin">"Lead Programmer"</span>
                            ", of Cebu Tours & Adventures, I was responsible for architecting and developing the full-stack system that powers the entire tour booking experience. My work spanned from building dynamic, component-based frontend interfaces to implementing a robust backend that handles real-time availability, inquiries, and booking flows."
                        </p>

                        <p class="mt-10 text-left text-lg font-normal tracking-wide text-neutral-600 dark:text-neutral-200">
                            "One of the major challenges was ensuring a seamless user journey across dozens of destinations and activities while keeping page speed and SEO in check. From adventure bookings to personalized tour recommendations, the system was designed with scalability and user engagement at its core."
                        </p>
                    </div>

                    <div class="relative bg-[url('/public/assets/works/cta/img-01.png')] h-96 lg:h-full bg-contain bg-center bg-no-repeat"></div>
                </div>
            </section>

            <div class="relative mx-auto my-12 grid grid-cols-2 lg:grid-cols-5 gap-10 lg:max-w-7xl xl:max-w-6xl justify-center">
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#1b468f] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$polynesian blue"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#0094d9] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$celestial blue"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#9CCFF0] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$light sky blue"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#dff3fc] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$alice blue"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center"></div>
            </div>

            <div class="relative mx-auto my-12 grid lg:grid-cols-2 gap-18 lg:gap-10 lg:max-w-7xl xl:max-w-6xl text-stone-900 dark:text-stone-100">
                <div class="space-y-8 text-center font-montserrat">
                    <h1 class="text-4xl">"Montserrat"</h1>

                    <div class="space-y-4 text-5xl">
                        <p>"a   b   c   d   e   f   g   h   i   j   k   l"</p>
                        <p>"m   n   o   p   q   r   s   t   u   v   w   x"</p>
                        <p>"y   z   1   2   3   4   5   6   7   9   0"</p>
                    </div>
                </div>

                <div class="space-y-8 text-center font-lora">
                    <h1 class="text-4xl">"Lora"</h1>

                    <div class="space-y-4 text-5xl">
                        <p>"a   b   c   d   e   f   g   h   i   j   k   l"</p>
                        <p>"m   n   o   p   q   r   s   t   u   v   w   x"</p>
                        <p>"y   z   1   2   3   4   5   6   7   9   0"</p>
                    </div>
                </div>
            </div>

            <div class="relative mx-auto mt-16 flex min-h-screen flex-col gap-1">
                <h3 class="text-center font-medium tracking-wide text-sky-600 dark:text-sky-400">
                    "UI & COMPONENTS"
                </h3>
                <h1 class="text-center text-3xl font-bold tracking-widest text-stone-600 dark:text-stone-300 capitalize">
                    "design"
                </h1>
                <div class="relative w-full h-screen max-w-6xl pt-20 mx-auto">

                    <div class="absolute top-12 left-1/2 -translate-x-1/2 bg-center bg-no-repeat bg-contain bg-[url('/public/assets/works/cta/img-02.png')] h-[40rem] w-2xl"></div>
                    <div class="absolute bottom-10 left-1/2 -translate-x-1/2 bg-center bg-contain bg-no-repeat bg-[url('/public/assets/works/cta/img-03.png')] h-[30rem] w-full"></div>
                </div>
            </div>
        </section>

        <NextWorkButton
            title="castlebyte techsolutions"
            link="/case-studies/castlebyte-techsolutions"
            color1="bg-[#2a659a]"
            color2="bg-[#ff7f00]"
        />
    }
}
