use leptos::{component, view, IntoView};

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="container mt-16 mx-auto">
            <h1 class="text-pink-500 text-2xl font-semibold">"Kate Korsaro"</h1>
            <p class="text-lg">A messy dev</p>
        </div>
    }
}
