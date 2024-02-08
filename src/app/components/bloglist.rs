use crate::*;

#[component]
pub fn BlogList() -> impl IntoView {
    view! {
        {
            let blogposts = list_of_all_blogposts();
            blogposts
                .into_iter()
                .map(|blogpost| {
                    view! {
                        <h1 class="dark:text-rose-500 text-rose-900 text-xl font-semibold">{&blogpost.title}</h1>
                        <p class="dark:text-slate-400 text-slate-600 text-xs mb-4">"published on: " {&blogpost.formatted_date()}</p>
                        <p class="mb-16">{blogpost.text}</p>
                    }
                })
                .collect_view()
        }
    }
}
