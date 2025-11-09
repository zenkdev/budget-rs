use web_sys::HtmlDialogElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DialogProps {
    pub open: bool,
    pub children: Children,
}

#[function_component]
pub fn Dialog(props: &DialogProps) -> Html {
    let DialogProps { open, children } = props;

    let dialog_ref = use_node_ref();
    let prev = use_mut_ref(|| *open);

    {
        let dialog_ref = dialog_ref.clone();
        let open = open.clone();

        use_effect_with((dialog_ref, open, prev), |(dialog_ref, open, prev)| {
            let dialog = dialog_ref
                .cast::<HtmlDialogElement>()
                .expect("dialog_ref not attached to dialog element");

            tracing::info!("open: {}, prev: {}", *open, *prev.borrow_mut());
            if *prev.borrow_mut() != *open {
                *prev.borrow_mut() = *open;
                if *open {
                    dialog.show_modal().unwrap();
                } else {
                    dialog.close();
                }
            }
        });
    }

    html! {
         <dialog ref={dialog_ref} class="bg-background-dark font-display min-h-screen w-full p-4 sm:p-6 md:p-8 text-primary">
            { if *open {
                html! {
                    { children.clone() }
                }
            } else {
                html! {}
            }}
         </dialog>
    }
}
