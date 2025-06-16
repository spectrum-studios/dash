use leptos::prelude::*;

#[component]
pub fn NotFoundView() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center w-full h-full">
            <div class="flex flex-col gap-2 items-center p-4 rounded-lg bg-zinc-800">
                <a href="/" class="self-start text-xs cursor-pointer">
                    "< Back"
                </a>
                <p class="text-8xl">"404"</p>
                <p>"Not Found :("</p>
            </div>
        </div>
    }
}
