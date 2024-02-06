mod hero;

use leptos::{component,view,IntoView};
use hero::*;

#[component]
pub fn App() -> impl IntoView {
    view! { <Hero/> }
}
