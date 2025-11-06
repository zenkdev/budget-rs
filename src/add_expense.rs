use web_sys::HtmlDialogElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AddExpenseProps {
    pub open: bool,
    pub on_close: Callback<MouseEvent>,
}

#[function_component]
pub fn AddExpense(props: &AddExpenseProps) -> Html {
    let AddExpenseProps { open, on_close } = props;
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
        <div class="layout-container flex h-full grow flex-col">
            <div class="flex flex-1 justify-center py-5">
                <div class="layout-content-container flex w-full max-w-3xl flex-col flex-1">
                    <div class="flex flex-wrap justify-center gap-3 p-4">
                        <p class="text-primary text-3xl md:text-4xl font-black leading-tight tracking-[-0.033em]">{"[ < NEW EXPENSE LOG > ]"}</p>
                    </div>
                    <form class="flex flex-col gap-6 mt-8">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
                            <div class="flex flex-col">
                                <label class="text-base font-medium leading-normal pb-2" for="amount">{"AMOUNT:"}</label>
                                <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal" id="amount" placeholder="&gt; $ [___]" type="number"/>
                            </div>
                            <div class="flex flex-col">
                                <label class="text-base font-medium leading-normal pb-2" for="date">{"DATE:"}</label>
                                <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal" id="date" placeholder="YYYY-MM-DD" type="date"/>
                            </div>
                        </div>
                        <div class="flex flex-col">
                            <label class="text-base font-medium leading-normal pb-2" for="description">{"DESCRIPTION:"}</label>
                            <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal" id="description" placeholder="Enter expense name..."/>
                        </div>
                        <div class="flex flex-col">
                            <label class="text-base font-medium leading-normal pb-2" for="category">{"CATEGORY:"}</label>
                            <select class="form-select appearance-none w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 p-4 text-base font-normal leading-normal bg-no-repeat bg-right" id="category" style="background-image: url(&quot;data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%2339ff14' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e&quot;); background-position: right 0.5rem center; background-size: 1.5em 1.5em;">
                                <option>{"Select category..."}</option>
                                <option>{"[FOOD]"}</option>
                                <option>{"[UTILITIES]"}</option>
                                <option>{"[TRANSPORT]"}</option>
                                <option>{"[ENTERTAINMENT]"}</option>
                                <option>{"[RENT]"}</option>
                                <option>{"[TRAVEL]"}</option>
                            </select>
                        </div>
                        <div class="flex flex-col">
                            <label class="text-base font-medium leading-normal pb-2" for="notes">{"NOTES:"}</label>
                            <textarea class="form-textarea flex w-full min-w-0 flex-1 resize-y overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-32 placeholder:text-primary/50 p-4 text-base font-normal leading-normal" id="notes" placeholder="Add optional details here..."></textarea>
                        </div>
                        <div class="flex flex-col sm:flex-row items-center justify-center gap-4 mt-8">
                            <button class="w-full sm:w-auto px-8 py-3 bg-transparent border border-primary text-primary font-bold hover:bg-primary/20 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 focus:ring-offset-background-dark transition-colors duration-200" type="submit">
                                {"[ SAVE ENTRY ]"}
                            </button>
                            <button class="w-full sm:w-auto px-8 py-3 text-primary/70 font-normal hover:text-primary focus:outline-none focus:ring-2 focus:ring-primary/50 focus:ring-offset-2 focus:ring-offset-background-dark transition-colors duration-200" type="button" onclick={on_close}>
                                {"[ CANCEL ]"}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    </dialog>
    }
}
