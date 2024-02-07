use leptos::{component, view, IntoView};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="container mx-auto mt-8">
            <h1 class="text-lg">"This is not the page you're looking for..."</h1>
        </div>
    }
}
