use leptos::prelude::*;

#[component]
pub fn HomeView() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center w-full h-full">
            <div class="flex flex-col gap-2 items-center p-4 rounded-lg bg-zinc-800 w-[20vw]">
                <p class="pb-2 text-2xl">"Dash by Spectrum Studios"</p>
                <a
                    href="/login"
                    class="p-2 w-full text-center text-white bg-blue-500 rounded-lg cursor-pointer hover:bg-blue-600"
                >
                    "Login"
                </a>
                <a
                    href="/register"
                    class="p-2 w-full text-center text-white rounded-lg border border-blue-500 cursor-pointer hover:bg-zinc-700"
                >
                    "Register"
                </a>
            </div>
        </div>
    }
}
