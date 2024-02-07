use crate::*;

#[component]
pub fn Home() -> impl IntoView {
    let (_state, set_state) = use_state();
    set_state.update(|x| x.navigation.selected_tab = SelectedTab::Home);
    view! {
        <Nav/>
        <Hero/>
        <AboutMe/>
        <Projects/>
    }
}
