use crate::*;

#[component]
pub fn ProjectCard(project: ReadSignal<Project>) -> impl IntoView {
    view! {
        <div
            class=("hover:border-red-500", move || project.get().status_tag == ProjectStatus::Red)
            class=("hover:border-amber-500", move || project.get().status_tag == ProjectStatus::Amber)
            class=("hover:border-lime-500", move || project.get().status_tag == ProjectStatus::Emerald)
            class=("hover:border-cyan-500", move || project.get().status_tag == ProjectStatus::Cyan)
            class="dark:bg-slate-700 dark:border-slate-700 bg-slate-300 border-slate-300 rounded p-2 border-s-4 shadow-sm hover:shadow-md"
        >
            <!-- "card title" -->
            <h2
                class="text-slate-800 dark:text-slate-200 text-lg font-semibold"
                class=("underline", move || project.get().project_url.is_some())
            >
                <a href=move || {
                    project.get().project_url.clone()
                }>{move || project.get().title.clone()}</a>
            </h2>
            <!-- "status" -->
            <p class="text-slate-600 dark:text-slate-400 text-xs">
                "Status: " {move || project.get().status.clone()}
            </p>
            <!-- "description" -->
            <p class="mt-2">{move || project.get().description.clone()}</p>
            <!-- "tags" -->
            <p class="text-slate-500 mt-2 text-xs">

                {move || {
                    project
                        .get()
                        .tags
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|x| format!("#{}", x))
                        .reduce(|acc, e| format!("{acc} {e}"))
                }}

            </p>
        </div>
    }
}
