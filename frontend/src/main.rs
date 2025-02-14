use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center h-screen bg-zinc-900 text-zinc-50">
            "Dash by Spectrum Studios"
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount::mount_to_body(App)
}
