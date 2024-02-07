use crate::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="container mt-8 mx-auto">
            <h1 class="text-pink-500 font-semibold text-xl">"Current Projects"</h1>
            <div class="grid gridcols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 mt-4">

                {
                    let projects = list_of_all_projects();
                    projects
                        .into_iter()
                        .map(|project| create_signal(project))
                        .map(|(read, _)| view! { <ProjectCard project=read/> })
                        .collect_view()
                }

            </div>
        </div>
    }
}

fn list_of_all_projects() -> Vec<Project> {
    let mut projects = vec![
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
    ];

    projects.sort_by(|a, b| a.title.cmp(&b.title));
    projects
}
