use leptos::prelude::*;

#[component]
pub fn CebuToursAdventuresView() -> impl IntoView {
    view! {
        <section
            class="min-h-[580px] bg-center bg-cover bg-no-repeat flex flex-col items-center justify-center gap-5"
            style="background-image: linear-gradient(to top, rgba(0,0,0,0.8) 0%, rgba(0,0,0,0.7) 100%), url('/public/assets/cebu-tours-adventures.png')"
        >
            <div class="flex flex-col gap-1 flex-1 items-center justify-center">
                <h1 class="text-[3.9975843648em] font-extrabold text-neutral-100 capitalize text-center">
                    "Cebu Tours & Adventure"
                </h1>
                <h3 class="text-[1.125em] italic text-neutral-100 font-normal max-w-xl text-center">
                    "Experience the beauty of life, one unforgettable adventure at a time in Cebu."
                </h3>
            </div>

            <div class="flex items-center justify-between pb-10 w-full px-12">
                <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                    <h1 class="text-sky-400">"role"</h1>
                    <h3 class="text-neutral-100">"front-end developer"</h3>
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
        </section>
    }
}
