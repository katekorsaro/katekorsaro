mod project_card;

use crate::{Project, ProjectStatus};
use leptos::{component, create_signal, view, IntoView, For};
use project_card::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="container mt-8 mx-auto">
            <h1 class="text-lg text-pink-500 font-semibold">"Current Projects"</h1>
            <div class="grid grid-cols-3 gap-4 mt-4">
            <For
                each=list_of_all_projects
                key=|state| state.id.clone()
                let:project
            >
                <ProjectCard project=project/>
            </For>
            </div>
        </div>
    }
}

fn list_of_all_projects() -> Vec<Project> {
    let project = Project::new(
        0,
        "Budgr",
        "work in progress",
        "description",
        ProjectStatus::Emerald,
    );

    vec![project]
}
