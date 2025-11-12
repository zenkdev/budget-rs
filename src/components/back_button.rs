use crate::prelude::*;

#[function_component]
pub fn BackButton() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <button
            class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden h-10 px-4 bg-transparent text-[#93c893] hover:text-primary text-sm font-bold leading-normal tracking-[0.015em]"
            onclick={onclick}
        >
            <span class="truncate">
                { "[ < BACK ]" }
            </span>
        </button>
    }
}
