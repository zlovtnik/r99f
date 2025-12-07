use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="content has-text-centered">
                <p>"Built with Rust 🦀 + WASM + Leptos"</p>
            </div>
        </footer>
    }
}
