use leptos::prelude::*;

#[component]
pub fn GoldenYearsView() -> impl IntoView {
    view! {
        <section
            class="min-h-[580px] bg-center bg-cover bg-no-repeat flex flex-col items-center justify-center gap-5"
            style="background-image: linear-gradient(to top, rgba(0,0,0,0.8) 0%, rgba(0,0,0,0.7) 100%), url('/public/assets/goldenyears.png')"
        >
            <div class="flex flex-col gap-1 flex-1 items-center justify-center">
                <h1 class="text-[3.9975843648em] font-extrabold text-neutral-100 capitalize text-center">
                    "GoldenYears Carehome"
                </h1>
                <h3 class="text-[1.125em] italic text-neutral-100 font-normal max-w-xl text-center">
                    "From everyday tasks to emotional companionship, we tailor our services to meet your loved one's unique needs, all in the familiar comfort of home."
                </h3>
            </div>

            <div class="flex items-center justify-between pb-10 w-full px-12">
                <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                    <h1 class="text-sky-400">"role"</h1>
                    <h3 class="text-neutral-100">"lead developer"</h3>
                </div>
                <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                    <h1 class="text-sky-400">"context"</h1>
                    <h3 class="text-neutral-100">"team project"</h3>
                </div>
                <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                    <h1 class="text-sky-400">"period"</h1>
                    <h3 class="text-neutral-100">"early 2025"</h3>
                </div>
            </div>
        </section>

        <section class="mb-24 min-h-screen">

            <section class="relative flex py-24 w-full flex-col items-center justify-center bg-[#A37C20]">
                <div class="flex max-w-xl flex-col items-center justify-center gap-8">
                    <h1 class="text-7xl font-black text-stone-100 capitalize">"the project"</h1>
                    <p class="text-center text-stone-200">
                        "We are located at Golden Years Care, and we consider that everybody deserves to age in a dignified, loving, and meaningful way. We have our values, on the basis of which all the interaction takes place, and every resident gets the specific support that a resident has a right to, both medical and emotional."
                    </p>
                    <a
                        href="https://goldenyears.cebutouradventures.com"
                        class="text-stone-100 border py-2 px-5 hover:bg-stone-100 hover:text-stone-800 duration-300 transition-all ease-in-out hover:border-stone-100 text-xl font-medium"
                    >
                        "Visit Website"
                    </a>
                </div>

                <div class="absolute bottom-0 right-10 z-0">
                    <h1 class="text-7xl font-montserrat font-black tracking-widest text-stone-100/30">
                        "GoldenYears Carehome"
                    </h1>
                </div>
            </section>

            <section class="relative overflow-hidden">
                <div class="mx-auto grid h-full grid-cols-1 gap-12 px-3 py-12 lg:max-w-7xl lg:grid-cols-2 lg:px-0 xl:max-w-6xl">
                    <div class="flex flex-col gap-2">
                        <h1 class="case-project-title">"cebu tours & adventures"</h1>
                        <p class="mt-10 text-left text-lg font-normal tracking-wide text-neutral-600 dark:text-neutral-200">
                            "My task as the Lead"
                            <span class="text-pumpkin">" Full-Stack Developer"</span>
                            " was to create a friendly, open communicative and informative site, which will promote the GoldenYears CareHome - a residence facility aimed at encouraging the elders with comfort, respect, and professional services."
                        </p>

                        <p class="mt-10 text-left text-lg font-normal tracking-wide text-neutral-600 dark:text-neutral-200">
                            "I was engaged in the entire technical gamut: the frontend had to be smooth and welcoming with a design that aligned with the mission of the care home, whereas the backend had to be able to host inquiries, content management, and availability in real-time of the services and amenities provided by the care home."
                        </p>
                    </div>

                    <div class="relative bg-[url('/public/assets/works/goldenyears/img-01.png')] bg-contain bg-center bg-no-repeat"></div>
                </div>
            </section>

            <div class="relative mx-auto my-12 grid sm:grid-cols-2 lg:grid-cols-5 gap-10 lg:max-w-7xl xl:max-w-6xl">
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#A37C20] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$dark goldenrod"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#6B5B2D] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$field drab"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#005D99] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$lapiz lazuli"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#F5E6C8] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$champagne"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#FAFAFA] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$seasalt"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center"></div>
            </div>

            <div class="relative mx-auto my-12 grid grid-cols-2 gap-10 lg:max-w-7xl xl:max-w-6xl text-stone-900 dark:text-stone-100">
                <div class="space-y-8 text-center font-cinzel">
                    <h1 class="text-4xl">"Cinzel"</h1>

                    <div class="space-y-4 text-5xl">
                        <p>"a   b   c   d   e   f   g   h   i   j   k   l"</p>
                        <p>"m   n   o   p   q   r   s   t   u   v   w   x"</p>
                        <p>"y   z   1   2   3   4   5   6   7   9   0"</p>
                    </div>
                </div>

                <div class="space-y-8 text-center font-garamond">
                    <h1 class="text-4xl">"garamond"</h1>

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

                    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-center bg-contain bg-[url('/public/assets/works/goldenyears/img-02.png')] h-full w-2xl"></div>
                    <div class="absolute bottom-10 left-1/2 -translate-x-1/2 bg-center bg-contain bg-no-repeat bg-[url('/public/assets/works/goldenyears/img-03.png')] h-[30rem] w-full"></div>
                </div>
            </div>
        </section>
    }
}
