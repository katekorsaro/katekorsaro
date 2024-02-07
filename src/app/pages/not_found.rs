use crate::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Nav/>
        <div class="container mx-auto mt-16 text-center">
            <h1 class="text-lg text-rose-500">"This is not the page you're looking for..."</h1>
        </div>
    }
}
