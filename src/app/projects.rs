mod project_card;

use crate::{Project, ProjectStatus};
use leptos::{component, create_signal, view, IntoView};
use project_card::*;

#[component]
pub fn Projects() -> impl IntoView {
    let project = Project::new(
        "Budgr",
        "work in progress",
        "description",
        ProjectStatus::Emerald,
    );

    let (project, _) = create_signal(project);

    view! {
        <div class="container mt-8 mx-auto">
            <h1 class="text-lg text-pink-500 font-semibold">"Current Projects"</h1>
            <div class="grid grid-cols-3 gap-4 mt-4">
                <ProjectCard project=project/>
                <ProjectCard project=project/>
                <ProjectCard project=project/>
                <ProjectCard project=project/>
                <ProjectCard project=project/>
            </div>
        </div>
    }
}
