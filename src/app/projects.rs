mod project_card;

use leptos::{component, create_signal, view, IntoView};
use project_card::*;

#[component]
pub fn Projects() -> impl IntoView {
    let (title, _) = create_signal(String::from("Budgr"));
    let (status, _) = create_signal(String::from("work in progress"));
    let (description, _) = create_signal(String::from(
        "this is a project description, hopefully on two lines...",
    ));

    view! {
        <div class="container mt-8 mx-auto">
            <h1 class="text-lg text-pink-500 font-semibold">"Current Projects"</h1>
            <div class="grid grid-cols-3 gap-4 mt-4">
                <ProjectCard title=title status=status description=description/>
                <ProjectCard title=title status=status description=description/>
                <ProjectCard title=title status=status description=description/>
            </div>
        </div>
    }
}
