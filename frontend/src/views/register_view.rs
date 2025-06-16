use leptos::prelude::*;

#[component]
pub fn RegisterView() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center w-full h-full">
            <div class="flex flex-col gap-4 items-center p-4 rounded-lg bg-zinc-800 w-[25vw]">
                <a href="/" class="self-start text-xs cursor-pointer">
                    "< Back"
                </a>
                <p class="pb-2 text-2xl">"Create a New Account"</p>
                <input
                    type="text"
                    placeholder="Username"
                    class="p-2 w-full text-white rounded border border-zinc-600 bg-zinc-700"
                />
                <input
                    type="email"
                    placeholder="Email"
                    class="p-2 w-full text-white rounded border border-zinc-600 bg-zinc-700"
                />
                <input
                    type="password"
                    placeholder="Password"
                    class="p-2 w-full text-white rounded border border-zinc-600 bg-zinc-700"
                />
                <button class="p-2 w-full text-white bg-blue-500 rounded-lg cursor-pointer hover:bg-blue-600">
                    "Register"
                </button>
                <p class="text-xs">
                    "Already have an account? "<a href="/login" class="underline">
                        "Login here"
                    </a>
                </p>
            </div>
        </div>
    }
}
