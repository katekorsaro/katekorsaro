use leptos::{component, view, IntoView};

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <div class="container mx-auto text-right mt-4">
            <ul>
                <li class="inline-block underline bg-slate-800 p-2 rounded rounded-2 shadow">
                    <a href="/">"Home"</a>
                </li>
                <li class="inline underline ml-4">
                    <a href="/blog">"Blog"</a>
                </li>
            </ul>
        </div>
    }
}
