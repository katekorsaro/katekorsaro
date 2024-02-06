mod app;

use leptos::{view};
use app::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
