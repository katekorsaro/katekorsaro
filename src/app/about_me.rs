use leptos::{component,view,IntoView};

#[component]
pub fn AboutMe () -> impl IntoView {
    view! {
        <div class="container mt-8 mx-auto">
            <h1 class="text-lg text-pink-500">"About Me"</h1>
            <ul class="list-disc ms-4">
                <li>"Kate Korsaro here, obviously a fake name :)"</li>
            </ul>
        </div>
    }
}
