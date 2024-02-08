use crate::*;

#[component]
pub fn BlogList() -> impl IntoView {
    view! {
        {
            let mut blogposts = list_of_all_blogposts();
            blogposts.sort_by(|a, b| b.date.cmp(&a.date));
            blogposts
                .iter()
                .map(|blogpost| {
                    view! {
                        <h1 class="dark:text-rose-500 text-rose-900 text-xl font-semibold">
                            {&blogpost.title}
                        </h1>
                        <p class="dark:text-slate-400 text-slate-600 text-xs mb-4">
                            "published on: " {&blogpost.formatted_date()}
                        </p>

                        {&blogpost
                            .text
                            .split("\n")
                            .map(|split| {
                                let line = String::from(split);
                                view! { <p class="mt-2">{line}</p> }
                            })
                            .collect_view()}
                    }
                })
                .collect_view()
        }
    }
}
