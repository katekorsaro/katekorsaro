use crate::*;

#[component]
pub fn Blog() -> impl IntoView {
    let (_state, set_state) = use_state();
    set_state.update(|x| x.navigation.selected_tab = SelectedTab::Blog);
    view! {
        <Nav/>
        <div class="container mt-16 mx-auto text-center">
            <h1 class="text-3xl text-rose-500">Coming soon...</h1>
        </div>
    }
}
