use leptos::{mount_to_body, prelude::*, view};

fn main() {
    mount_to_body(|| {
        view! {
             <h1 class="canttouchthis">"Hello, World!" </h1>
        }
    })
}
