use web_sys::HtmlDialogElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ManageBudgetsProps {
    pub open: bool,
    pub on_close: Callback<MouseEvent>,
}

#[function_component]
pub fn ManageBudgets(props: &ManageBudgetsProps) -> Html {
    let ManageBudgetsProps { open, on_close } = props;
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
            <div class="px-4 sm:px-10 md:px-20 lg:px-40 flex flex-1 justify-center py-5">
                <div class="layout-content-container flex flex-col max-w-[960px] flex-1">
                    // <!-- Back Button -->
                    <div class="flex px-4 py-3 justify-start">
                        <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden h-10 px-4 bg-transparent text-[#93c893] hover:text-primary text-sm font-bold leading-normal tracking-[0.015em]" onclick={on_close}>
                            <span class="truncate">{"[ < BACK ]"}</span>
                        </button>
                    </div>
                    // <!-- Page Heading -->
                    <div class="flex flex-wrap justify-between gap-3 p-4">
                        <div class="flex min-w-72 flex-col gap-3">
                            <p class="text-primary text-4xl font-black leading-tight tracking-[-0.033em]">{"[ BUDGET LIMIT CONFIGURATION ]"}</p>
                            <p class="text-[#93c893] text-base font-normal leading-normal">{"> SET SPENDING THRESHOLDS. SYSTEM WILL ALERT WHEN LIMITS ARE APPROACHED."}</p>
                        </div>
                    </div>
                    // <!-- Overall Limit Input -->
                    <div class="flex max-w-[480px] flex-wrap items-end gap-4 px-4 py-3">
                        <label class="flex flex-col min-w-40 flex-1">
                            <p class="text-primary text-base font-medium leading-normal pb-2">{"> OVERALL MONTHLY LIMIT:"}</p>
                            <div class="relative">
                                <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden text-primary focus:outline-0 focus:ring-0 border border-[#346534] bg-[#1a321a] focus:border-primary h-14 placeholder:text-[#93c893] p-[15px] text-base font-normal leading-normal pr-4" value="$2000.00"/>
                                <span class="blinking-cursor absolute right-3 top-1/2 -translate-y-1/2 text-primary font-bold">{"_"}</span>
                            </div>
                        </label>
                    </div>
                    // <!-- Spacer -->
                    <div class="h-8"></div>
                    // <!-- Category Limits Table -->
                    <div class="px-4 py-3 @container">
                        <div class="flex overflow-hidden border border-[#346534] bg-background-dark">
                            <table class="flex-1 w-full">
                                <thead>
                                    <tr class="bg-[#1a321a]">
                                        <th class="px-4 py-3 text-left text-primary w-[30%] sm:w-[25%] text-sm font-medium leading-normal">{"CATEGORY"}</th>
                                        <th class="px-4 py-3 text-left text-primary w-[30%] sm:w-[25%] text-sm font-medium leading-normal">{"LIMIT"}</th>
                                        <th class="px-4 py-3 text-left text-primary w-[40%] sm:w-[50%] text-sm font-medium leading-normal">{"CURRENT SPEND"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr class="border-t border-t-[#346534]">
                                        <td class="h-[72px] px-4 py-2 text-[#93c893] text-sm font-normal leading-normal">{"[GROCERIES]"}</td>
                                        <td class="h-[72px] px-4 py-2">
                                            <input class="form-input w-full min-w-0 resize-none overflow-hidden text-primary focus:outline-0 focus:ring-0 border border-[#346534] bg-[#1a321a] focus:border-primary h-12 placeholder:text-[#93c893] p-3 text-sm font-normal leading-normal" value="$500.00"/>
                                        </td>
                                        <td class="h-[72px] px-4 py-2 text-sm font-normal leading-normal">
                                            <p class="text-primary text-sm font-medium leading-normal">{"[█████████░] 90%"}</p>
                                        </td>
                                    </tr>
                                    <tr class="border-t border-t-[#346534]">
                                        <td class="h-[72px] px-4 py-2 text-[#93c893] text-sm font-normal leading-normal">{"[UTILITIES]"}</td>
                                        <td class="h-[72px] px-4 py-2">
                                            <input class="form-input w-full min-w-0 resize-none overflow-hidden text-primary focus:outline-0 focus:ring-0 border border-[#346534] bg-[#1a321a] focus:border-primary h-12 placeholder:text-[#93c893] p-3 text-sm font-normal leading-normal" value="$200.00"/>
                                        </td>
                                        <td class="h-[72px] px-4 py-2 text-sm font-normal leading-normal">
                                            <p class="text-primary text-sm font-medium leading-normal">{"[█████░░░░░] 50%"}</p>
                                        </td>
                                    </tr>
                                    <tr class="border-t border-t-[#346534]">
                                        <td class="h-[72px] px-4 py-2 text-[#93c893] text-sm font-normal leading-normal">{"[TRANSPORT]"}</td>
                                        <td class="h-[72px] px-4 py-2">
                                            <input class="form-input w-full min-w-0 resize-none overflow-hidden text-primary focus:outline-0 focus:ring-0 border border-[#346534] bg-[#1a321a] focus:border-primary h-12 placeholder:text-[#93c893] p-3 text-sm font-normal leading-normal" value="$150.00"/>
                                        </td>
                                        <td class="h-[72px] px-4 py-2 text-sm font-normal leading-normal">
                                            <p class="text-primary text-sm font-medium leading-normal">{"[██░░░░░░░░] 25%"}</p>
                                        </td>
                                    </tr>
                                    <tr class="border-t border-t-[#346534]">
                                        <td class="h-[72px] px-4 py-2 text-[#93c893] text-sm font-normal leading-normal">{"[ENTERTAINMENT]"}</td>
                                        <td class="h-[72px] px-4 py-2">
                                            <input class="form-input w-full min-w-0 resize-none overflow-hidden text-primary focus:outline-0 focus:ring-0 border border-[#346534] bg-[#1a321a] focus:border-primary h-12 placeholder:text-[#93c893] p-3 text-sm font-normal leading-normal" value="$250.00"/>
                                        </td>
                                        <td class="h-[72px] px-4 py-2 text-sm font-normal leading-normal">
                                            <p class="text-primary text-sm font-medium leading-normal">{"[██████████] 100%"}</p>
                                        </td>
                                    </tr>
                                    <tr class="border-t border-t-[#346534]">
                                        <td class="h-[72px] px-4 py-2 text-[#93c893] text-sm font-normal leading-normal">{"[MISC]"}</td>
                                        <td class="h-[72px] px-4 py-2">
                                            <input class="form-input w-full min-w-0 resize-none overflow-hidden text-primary focus:outline-0 focus:ring-0 border border-[#346534] bg-[#1a321a] focus:border-primary h-12 placeholder:text-[#93c893] p-3 text-sm font-normal leading-normal" value="$400.00"/>
                                        </td>
                                        <td class="h-[72px] px-4 py-2 text-sm font-normal leading-normal">
                                            <p class="text-primary text-sm font-medium leading-normal">{"[█░░░░░░░░░] 10%"}</p>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                    // <!-- Spacer -->
                    <div class="h-8"></div>
                    // <!-- Action Buttons -->
                    <div class="flex justify-stretch">
                        <div class="flex flex-1 gap-3 flex-wrap px-4 py-3 justify-start">
                            <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden h-10 px-4 bg-primary text-background-dark hover:bg-opacity-80 text-sm font-bold leading-normal tracking-[0.015em]">
                                <span class="truncate">{"[ SAVE CHANGES ]"}</span>
                            </button>
                            <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden h-10 px-4 bg-[#244724] text-[#93c893] hover:text-primary text-sm font-bold leading-normal tracking-[0.015em]" onclick={on_close}>
                                <span class="truncate">{"[ RESET TO DEFAULTS ]"}</span>
                            </button>
                        </div>
                    </div>
                    // <!-- Status Line -->
                    <div class="px-4 py-6">
                        <p class="text-primary text-base font-normal leading-normal">{"STATUS: AWAITING USER INPUT"}<span class="blinking-cursor">{"_"}</span></p>
                    </div>
                </div>
            </div>
        </div>
    </dialog>
    }
}
