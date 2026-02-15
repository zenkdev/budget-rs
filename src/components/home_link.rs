use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
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

            // Wrap closure in Rc<RefCell<Option<>>> so we can reference it in both subscription and cleanup
            let closure = Rc::new(RefCell::new(None));
            let closure_clone = closure.clone();

            let callback = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                if e.key() == "Escape" {
                    e.prevent_default();
                    navigator.push(&Route::Home);
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);

            // Save into Rc<RefCell<Option<>>>
            *closure.borrow_mut() = Some(callback);

            // Get JsValue reference
            let js_value = closure.borrow().as_ref().unwrap().as_ref().clone();

            tracing::info!("subscribe keydown");
            window
                .add_event_listener_with_callback("keydown", js_value.unchecked_ref())
                .unwrap();

            // Cleanup closure on unmount
            move || {
                tracing::info!("unsubscribe keydown");
                if let Some(cb) = closure_clone.borrow_mut().take() {
                    window
                        .remove_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
                        .unwrap();
                }
            }
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
