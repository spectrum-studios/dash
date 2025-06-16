use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

mod views;

#[component]
fn App() -> impl IntoView {
    use views::*;

    view! {
        <Router>
            <div class="relative w-screen h-screen bg-zinc-900 text-zinc-50">
                <Routes fallback=not_found_view::NotFoundView>
                    <Route path=path!("/") view=home_view::HomeView />
                    <Route path=path!("/login") view=login_view::LoginView />
                    <Route path=path!("/register") view=register_view::RegisterView />
                </Routes>
            </div>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount::mount_to_body(App)
}
