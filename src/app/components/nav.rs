use crate::*;

#[component]
pub fn Nav() -> impl IntoView {
    let (state, _set_state) = use_state();
    let navs = vec![
        ("Home", &SelectedTab::Home, "/"),
        ("Blog", &SelectedTab::Blog, "/blog"),
    ];
    view! {
        <div class="container mx-auto text-right mt-4">
            <ul>

                {navs
                    .into_iter()
                    .map(|nav| {
                        view! {
                            <li
                                class="inline-block p-2 rounded"
                                class=(
                                    "underline",
                                    move || { state.with(|s| s.navigation.selected_tab == *nav.1) },
                                )

                                class=(
                                    "decoration-2",
                                    move || { state.with(|s| s.navigation.selected_tab == *nav.1) },
                                )

                                class=(
                                    "underline-offset-2",
                                    move || { state.with(|s| s.navigation.selected_tab == *nav.1) },
                                )

                                class=(
                                    "decoration-rose-900",
                                    move || { state.with(|s| s.navigation.selected_tab == *nav.1) },
                                )

                                class=(
                                    "dark:decoration-rose-500",
                                    move || { state.with(|s| s.navigation.selected_tab == *nav.1) },
                                )
                            >

                                <a href=nav.2>{nav.0}</a>
                            </li>
                        }
                    })
                    .collect_view()}

            </ul>
        </div>
    }
}
