use leptos::prelude::*;
use leptos_icons::Icon as LeptosIcon;
use leptos_router::components::A;

#[derive(Debug, Clone)]
enum BadgeStatus {
	Live,
	Onprogess,
	None,
}

#[component]
pub fn FeatureProjects() -> AnyView {
	let proj_1_tech = RwSignal::new(
		[
			"VueJS".to_string(),
			"NodeJS".to_string(),
			"Quasar".to_string(),
			"MySQL".to_string(),
			"Electron".to_string(),
		]
		.to_vec(),
	);

	let project_1_tech = RwSignal::new(
		[
			"Leptos (Rust)".to_string(),
			"Axum (Rust)".to_string(),
			"Postgres".to_string(),
			"Firebase".to_string(),
		]
		.to_vec(),
	);

	view! {
        <section id="projects" class="container py-20 px-4 mx-auto">
            <div>
                <h2 class="mb-2 text-3xl font-bold">"Feature Projects"</h2>
                <div class="mb-8 w-20 h-1 bg-primary"></div>
                <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                    <ProjectCard
                        img="/assets/projects/cpc-library.png"
                        tech=proj_1_tech
                        title="CPC Library"
                        description="A cross-platform application and user-friendly software designed to provide a comprehensive and efficient platform for managing and accessing library resources. With its intuitive design and powerful features, this platform makes it easy for librarians and patrons to search, browse, borrow, and return books and other materials"
                        source="https://github.com/icarusvagner/library-cpc-system"
                    />
                    <ProjectCard tech=project_1_tech />
                    <ProjectCard tech=project_1_tech />
                </div>
                <div class="mt-12 text-center">
                    <A
                        href="https://github.com/icarusvagner/"
                        attr:class="btn btn-primary btn-outline text-md"
                    >
                        "View All Projects on GitHub"
                        <LeptosIcon icon=icondata::FiExternalLink attr:class="h-4 w-4" />
                    </A>
                </div>
            </div>
        </section>
    }
    .into_any()
}

#[component]
fn ProjectCard(
	#[prop(into, optional)] title: MaybeProp<String>,
	#[prop(into, optional)] status: MaybeProp<BadgeStatus>,
	#[prop(into, optional)] description: MaybeProp<String>,
	#[prop(into, optional)] img: MaybeProp<String>,
	#[prop(into, optional)] source: MaybeProp<String>,
	#[prop(into)] tech: RwSignal<Vec<String>>,
) -> AnyView {
	view! {
		<div class="w-full shadow-sm card bg-base-100">
			<figure>
				<img
					src=move || img.get().unwrap_or("http://picsum.photos/1200/1200".to_string())
					alt=move || title.get().unwrap_or("Unknown Title".to_string())
					class="object-cover w-full h-96"
				/>
			</figure>
			<div class="card-body">
				<h2 class="card-title">
					{move || title.get().unwrap_or("Unknown Title".to_string())}
					<div class=move || {
						match status.get().unwrap_or(BadgeStatus::None) {
							BadgeStatus::Live => "badge badge-outline badge-success",
							BadgeStatus::Onprogess => "badge badge-outline badge-info",
							BadgeStatus::None => "",
						}
					}>
						{move || match status.get().unwrap_or(BadgeStatus::None) {
							BadgeStatus::Live => "Live",
							BadgeStatus::Onprogess => "On Progress",
							BadgeStatus::None => "",
						}}
					</div>
				</h2>
				<div class="space-y-3">
					<p class="text-base-content/90">
						{move || {
							description
								.get()
								.unwrap_or(
									"Lorem ipsum dolor sit amet consectetur adipiscing elit quisque faucibus."
										.to_string(),
								)
						}}
					</p>
					<div class="flex flex-wrap justify-between">
						<For each=move || tech.get() key=|tech| tech.clone() let:child>
							<p class="font-bold text-md">{child}</p>
						</For>
					</div>
				</div>
				<div class="justify-end card-actions">
					<A
						href=move || source.get().unwrap_or("/".to_string())
						attr:class="btn btn-outline"
						target="_blank"
					>
						"Source Code"
						<LeptosIcon icon=icondata::BiGithub attr:class="h-4 w-4" />
					</A>
				</div>
			</div>
		</div>
	}
	.into_any()
}
