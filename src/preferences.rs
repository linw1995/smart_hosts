use leptos::*;

use crate::utils::get_current_window;

#[component]
pub fn Preferences() -> impl IntoView {
    view! {
        <main class="w-screen h-screen rounded-xl bg-white flex flex-col">
            <div
                data-tauri-drag-region
                class="w-full h-11 px-4 flex gap-x-1 pt-3 justify-end items-center z-[99]"
            >
                <div className="font-bold text-[#D8D8D8] select-none">"Smart Hosts"</div>
                <div className="bg-[#D8D8D8] w-[2px] h-[16px] rounded-[10px] mx-2"></div>

                <JellyButton on:click=|_| get_current_window().close() />
            </div>

            <div class="mb-[8vh] grow flex items-center justify-center font-bold text-[#ebebeb] text-4xl select-none">
                ":)"
            </div>
        </main>
    }
}

#[component]
pub fn JellyButton() -> impl IntoView {
    use phosphor_leptos::{Icon, X};

    view! {
        <button class="w-[32px] h-[32px] bg-[#EE7D7D] flex items-center justify-center">
            <Icon icon=X />
        </button>
    }
}
