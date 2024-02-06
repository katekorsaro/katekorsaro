use leptos::{component,view,IntoView};
use crate::app::{Nav, Hero, AboutMe, Projects};

#[component]
pub fn Home () -> impl IntoView {
    view! {
        <Nav/>
        <Hero/>
        <AboutMe/>
        <Projects/>
    }
}
