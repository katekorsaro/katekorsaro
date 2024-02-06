mod project_card;

use crate::{Project, ProjectStatus};
use leptos::{component, create_signal, view, IntoView};
use project_card::*;

#[component]
pub fn Projects() -> impl IntoView {
    let project = Project {
        title: String::from("Budgr"),
        status: String::from("work in progress"),
        description: String::from("this is a long and hopefully multiline description!"),
        status_tag: ProjectStatus::Cyan,
    };

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
