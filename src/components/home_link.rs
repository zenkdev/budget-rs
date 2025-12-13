use crate::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

#[derive(PartialEq)]
pub enum HomeLinkVariant {
    Back,
    Close,
}

#[derive(PartialEq, Properties)]
pub struct HomeLinkProps {
    #[prop_or(HomeLinkVariant::Back)]
    pub variant: HomeLinkVariant,
}

#[function_component]
pub fn HomeLink(props: &HomeLinkProps) -> Html {
    let HomeLinkProps { variant } = props;
    let navigator = use_navigator().unwrap();
    {
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            let window = gloo::utils::window();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: KeyboardEvent| {
                if e.key() == "Escape" {
                    e.prevent_default();
                    navigator.push(&Route::Home);
                }
            })
                as Box<dyn FnMut(KeyboardEvent)>);

            window
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();

            || {}
        });
    }

    html! {
        <Link<Route>
            classes={classes!(
                "flex", "cursor-pointer", "items-center", "justify-center", "overflow-hidden", "bg-transparent", "text-[#93c893]", "hover:text-primary", "text-sm", "font-bold", "leading-normal", "tracking-[0.015em]",
                variant.eq(&HomeLinkVariant::Back).then_some("min-w-[84px] max-w-[480px]"),
            )}
            to={Route::Home}
        >
            { if variant.eq(&HomeLinkVariant::Back) {
                html! {
                    <span class="truncate">
                        { "[ < BACK ]" }
                    </span>
                }
            } else {
                html! {
                    <span class="text-2xl font-bold">
                        { "[X]" }
                    </span>
                }
            } }
        </Link<Route>>
    }
}
