use crate::*;

#[component]
pub fn AboutMe() -> impl IntoView {
    view! {
        <div class="container mt-8 mx-auto">
            <h1 class="text-rose-900 dark:text-rose-500 font-semibold text-xl">"About Me"</h1>

            <div class="columns-1 md:columns-2">
                <ul class="list-disc ms-4">
                    <li>"Kate Korsaro here, obviously a " <u>"fake name"</u> " :)"</li>
                    <li>
                        <u>"Rust"</u>
                        " programming language"
                    </li>
                    <li>"CLI utilities"</li>
                    <li>
                        <u>"Linux"</u>
                        " (currently running a heavy customised Debian 12)"
                    </li>
                    <li>"Web development (not much to be honest, but it's trendy)"</li>
                    <li>
                        <u>"Roleplaying"</u>
                        " games"
                    </li>
                    <li>"Working as a full time developer and team lead"</li>
                    <li>"And in my spare time"</li>
                </ul>
            </div>

        </div>
    }
}
