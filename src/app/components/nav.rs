use crate::*;

#[component]
pub fn Nav() -> impl IntoView {
    let (state, _set_state) = use_state();
    view! {
        <div class="container mx-auto text-right mt-4">
            <ul>
                <li
                    class="inline-block p-2 rounded"
                    class=("underline", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Home))
                    class=("decoration-2", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Home))
                    class=("underline-offset-2", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Home))
                    class=("decoration-rose-900", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Home))
                    class=("dark:decoration-rose-500", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Home))
                >

                    <a href="/">"Home"</a>
                </li>
                <li
                    class="inline-block p-2 rounded"
                    class=("underline", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Blog))
                    class=("decoration-2", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Blog))
                    class=("underline-offset-2", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Blog))
                    class=("decoration-rose-900", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Blog))
                    class=("dark:decoration-rose-500", move || state.with(|s| s.navigation.selected_tab == SelectedTab::Blog))
                >

                    <a href="/blog">"Blog"</a>
                </li>
            </ul>
        </div>
    }
}
