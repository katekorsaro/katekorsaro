use leptos::{component,view,IntoView};
use crate::app::components::{Nav};

#[component]
pub fn Blog () -> impl IntoView {
    view! {
        <Nav/>
        <div class="container mt-16 mx-auto text-center">
            <h1 class="text-3xl text-pink-500">Coming soon...</h1>
        </div>
    }
}
