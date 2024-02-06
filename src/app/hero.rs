use leptos::{component,view,IntoView};

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="container mt-16 mx-auto">
            <h1 class="text-pink-500 text-xl">"Kate Korsaro"</h1>
            <p>A messy dev</p>
        </div>
    }
}
