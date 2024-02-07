mod project_card;

use crate::{Project, ProjectStatus};
use leptos::{component, view, For, IntoView};
use project_card::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="container mt-8 mx-auto">
            <h1 class="text-pink-500 font-semibold">"Current Projects"</h1>
            <div class="grid grid-cols-3 gap-4 mt-4">
                <For each=list_of_all_projects key=|state| state.title.clone() let:project>
                    <ProjectCard project=project/>
                </For>
            </div>
        </div>
    }
}

fn list_of_all_projects() -> Vec<Project> {
    vec![
        Project::new(
            "budgr",
            "work in progress",
            "A CLI personal finance and budget manager heavily inspired by taskwarrior",
            ProjectStatus::Amber,
        )
        .tags(vec!["finance", "budget", "rust"])
        .project_url("https://github.com/katekorsaro/budgr"),
        Project::new(
            "budgr-tui",
            "not yet started",
            "A front-end for budgr using ratatui rust library",
            ProjectStatus::Red,
        )
        .tags(vec!["tui", "rust", "finance", "front-end"]),
        Project::new(
            "iron-dice",
            "usable",
            "A CLI dice roller written in rust. Highly configurable and fast to use",
            ProjectStatus::Emerald,
        )
        .tags(vec!["rpg", "random", "dice"])
        .project_url("https://github.com/katekorsaro/iron-dice"),
        Project::new(
            "2d6 tales",
            "barely started",
            "A light generic rpg system based on the 2d6 roll high mechanic",
            ProjectStatus::Red,
        )
        .tags(vec!["rpg", "game", "tabletop"])
        .project_url("https://github.com/katekorsaro/2d6-tales-srd"),
    ]
}
