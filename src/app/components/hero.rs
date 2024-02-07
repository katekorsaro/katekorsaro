use leptos::{component, view, IntoView};

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="container mt-16 mx-auto">
            <h1 class="text-pink-500 font-semibold text-xl">"Kate Korsaro"</h1>
            <p class="">A messy dev</p>
        </div>
    }
}
