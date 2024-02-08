use crate::*;

#[component]
pub fn Blog() -> impl IntoView {
    let (_state, set_state) = use_state();
    set_state.update(|x| x.navigation.selected_tab = SelectedTab::Blog);
    view! {
        <Nav/>
        <div class="container max-w-2xl mt-16 mx-auto">
            <BlogList/>
        </div>
    }
}
