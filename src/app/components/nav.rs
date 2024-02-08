use crate::*;

#[component]
pub fn Nav() -> impl IntoView {
    let (state, _set_state) = use_state();
    view! {
        <div class="container mx-auto text-right mt-4">
            <ul>
                <li
                    class="inline-block underline p-2 rounded rounded-4"
                    class=(
                        "bg-slate-300",
                        move || state.get().navigation.selected_tab == SelectedTab::Home,
                    )
                    class=(
                        "dark:bg-slate-700",
                        move || state.get().navigation.selected_tab == SelectedTab::Home,
                    )
                >

                    <a href="/">"Home"</a>
                </li>
                <li
                    class="inline-block underline p-2 rounded rounded-4"
                    class=(
                        "bg-slate-300",
                        move || state.get().navigation.selected_tab == SelectedTab::Blog,
                    )
                    class=(
                        "dark:bg-slate-700",
                        move || state.get().navigation.selected_tab == SelectedTab::Blog,
                    )
                >

                    <a href="/blog">"Blog"</a>
                </li>
            </ul>
        </div>
    }
}
