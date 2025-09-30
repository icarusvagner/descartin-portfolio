use leptos::prelude::*;

#[component]
pub fn ExperienceComponent() -> impl IntoView {
	let exp_1 = RwSignal::new(
		[
			"Lead a team developer building and maintaining the website".to_string(),
			"Optimized front-end performormance resulting in 40% faster page load times"
				.to_string(),
		]
		.to_vec(),
	);
	let exp_1_techs = RwSignal::new(
		[
			"Leptos CSR".to_string(),
			"Rust".to_string(),
			"WASM".to_string(),
		]
		.to_vec(),
	);

	let exp_2 = RwSignal::new(
		[
			"Dashboard optimization with Power BI".to_string(),
			"Implementing Power Query and Pivot functions in MSSQL".to_string(),
		]
		.to_vec(),
	);
	let exp_2_techs = RwSignal::new(
		[
			"Php".to_string(),
			"Power BI".to_string(),
			"MSSQL".to_string(),
		]
		.to_vec(),
	);

	let exp_3 = RwSignal::new(["Developing CPC Library System to cater the needs of students, faculty, and librarian".to_string(),
		"Comprehensive solution for library management, aiming to streamline various processes such as cataloging, circulation, acquisitions, and reporting.".to_string()
	].to_vec());
	let exp_3_techs = RwSignal::new(
		[
			"VueJS".to_string(),
			"Electron".to_string(),
			"MySQL".to_string(),
			"Node.JS".to_string(),
			"TypeScript".to_string(),
		]
		.to_vec(),
	);

	view! {
		<section id="experience" class="container px-4 mx-auto">
			<div>
				<h2 class="mb-2 text-3xl font-bold">"Work Experience"</h2>
				<div class="mb-8 w-20 h-1 bg-primary"></div>
				<div class="mx-auto max-w-3xl">
					<div class="relative mb-12">
						<ExperienceCard
							year="2025 - Present"
							company="Cebu Tours & Adventure"
							location="Lapu-lapu City, Cebu"
							job_title="I.T Specialist"
							job_desc=exp_1
							tech_stack=exp_1_techs
						/>
						<hr class="my-8 w-full h-0.5 border-none shrink-0 divider" />
						<ExperienceCard
							year="6 Months"
							company="DAC Innovative and IT Services"
							location="Ayala Business Park, Cebu"
							job_title="Web Developer"
							job_desc=exp_2
							tech_stack=exp_2_techs
						/>
						<hr class="my-8 w-full h-0.5 border-none shrink-0 divider" />
						<ExperienceCard
							year="6 Months"
							company="CPC Library"
							location="Gabi, Cordova, Cebu"
							job_title="Full-stack Developer"
							job_desc=exp_3
							tech_stack=exp_3_techs
						/>
					</div>
				</div>
			</div>
		</section>
	}
}

#[component]
fn ExperienceCard(
	#[prop(into)] year: String,
	#[prop(into)] company: String,
	#[prop(into)] location: String,
	#[prop(into)] job_title: String,
	#[prop(into)] job_desc: RwSignal<Vec<String>>,
	#[prop(into)] tech_stack: RwSignal<Vec<String>>,
) -> impl IntoView {
	view! {
		<div class="flex flex-col gap-4 md:flex-row">
			<div class="md:w-1/3">
				<div class="inline-flex relative shrink-0">{year}</div>
				<h3 class="text-xl font-semibold capitalize">{company}</h3>
				<p class="capitalize text-neutral/60">{location}</p>
			</div>
			<div class="md:w-2/3">
				<h2 class="mb-2 text-lg font-semibold">{job_title}</h2>
				<ul class="ml-5 space-y-2 list-disc list-outside text-base-content/70">
					<For each=move || job_desc.get() key=|job_desc| job_desc.clone() let:child>
						<li>{child}</li>
					</For>
				</ul>
				<div class="flex flex-wrap gap-2 mt-4">
					<For each=move || tech_stack.get() key=|tech| tech.clone() let:child>
						<div class="inline-flex relative shrink-0">{child}</div>
					</For>
				</div>
			</div>
		</div>
	}
}
