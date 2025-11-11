use crate::prelude::*;

#[derive(PartialEq)]
enum Tabs {
    Import,
    Export,
}

#[derive(PartialEq)]
enum Format {
    Csv,
    Json,
}

#[derive(PartialEq, Properties)]
pub struct DataModalProps {
    pub open: bool,
    pub on_close: Callback<()>,
}

#[function_component]
pub fn DataModal(props: &DataModalProps) -> Html {
    let DataModalProps { open, on_close } = props;
    let handle_close = {
        let on_close = on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };
    let current_tab = use_state(|| Tabs::Import);
    let on_import_tab_click = {
        let current_tab = current_tab.clone();
        Callback::from(move |_| current_tab.set(Tabs::Import))
    };
    let on_export_tab_click = {
        let current_tab = current_tab.clone();
        Callback::from(move |_| current_tab.set(Tabs::Export))
    };
    let format = use_state(|| Format::Csv);
    let on_csv_click = {
        let format = format.clone();
        Callback::from(move |_| format.set(Format::Csv))
    };
    let on_json_click = {
        let format = format.clone();
        Callback::from(move |_| format.set(Format::Json))
    };

    let state = use_context::<State>().expect("no ctx found");
    let on_initiate_click = {
        if (*current_tab).eq(&Tabs::Import) {
            let on_close = on_close.clone();
            Callback::from(move |_| on_close.emit(()))
        } else {
            let on_close = on_close.clone();
            let transactions = state.transactions.clone();
            let categories = state.categories.clone();
            Callback::from(move |_| {
                save_data_as_csv_file(transactions.clone(), categories.clone());
                on_close.emit(())
            })
        }
    };

    html! {
        <Dialog open={open}>
            <div class="mx-auto flex h-full w-full max-w-[960px] grow flex-col border-2 border-primary/20 bg-[#112211] rounded-lg">
                <div class="flex flex-col flex-1 p-4 md:p-6 lg:p-8">
                    <div class="flex flex-wrap justify-between gap-3 items-start">
                        <div class="flex min-w-72 flex-col gap-3">
                            <p class="text-white tracking-light text-2xl md:text-[32px] font-bold leading-tight">
                                { "// DATA TRANSFERENCE INTERFACE //" }
                            </p>
                            <p class="text-[#93c893] text-sm font-normal leading-normal">
                                { "AWAITING COMMAND..." }
                            </p>
                        </div>
                        <button class="flex items-center justify-center text-[#93c893] hover:text-primary transition-colors" onclick={handle_close.clone()}>
                            <span class="text-2xl font-bold">
                                { "[X]" }
                            </span>
                        </button>
                    </div>
                    <div class="pt-4 pb-3">
                        <div class="flex border-b border-[#346534] gap-8">
                            <button
                                class={classes!(
                                    "flex", "flex-col", "items-center", "justify-center", "border-b-[3px]", "pb-[13px]", "pt-4",
                                    (*current_tab).eq(&Tabs::Import).then_some("border-b-[#19e619]").or(Some("border-b-transparent")),
                                    (*current_tab).eq(&Tabs::Import).then_some("text-white").or(Some("text-[#93c893] hover:text-white")),
                                )}
                                onclick={on_import_tab_click}
                            >
                                <p
                                    class={classes!(
                                        "text-sm", "font-bold", "leading-normal", "tracking-[0.015em]",
                                        (*current_tab).eq(&Tabs::Import).then_some("text-white").or(Some("text-[#93c893] hover:text-white")),
                                    )}
                                >
                                    { "[ IMPORT ]" }
                                </p>
                            </button>
                            <button
                                class={classes!(
                                    "flex", "flex-col", "items-center", "justify-center", "border-b-[3px]", "pb-[13px]", "pt-4",
                                    (*current_tab).eq(&Tabs::Export).then_some("border-b-[#19e619]").or(Some("border-b-transparent")),
                                    (*current_tab).eq(&Tabs::Export).then_some("text-white").or(Some("text-[#93c893] hover:text-white")),
                                )}
                                onclick={on_export_tab_click}
                            >
                                <p
                                    class={classes!(
                                        "text-sm", "font-bold", "leading-normal", "tracking-[0.015em]",
                                        (*current_tab).eq(&Tabs::Export).then_some("text-white").or(Some("text-[#93c893] hover:text-white")),
                                    )}
                                >
                                    { "[ EXPORT ]" }
                                </p>
                            </button>
                        </div>
                    </div>
                    <div class="flex flex-col flex-1">
                        <h2 class="text-white text-[22px] font-bold leading-tight tracking-[-0.015em] pb-3 pt-5">
                            { if (*current_tab).eq(&Tabs::Import) {
                                "> INBOUND DATA STREAM"
                            } else {
                                "> OUTBOUND DATA STREAM"
                            } }
                        </h2>
                        <h3 class="text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-3 pt-5">
                            { "> SELECT FORMAT:" }
                        </h3>
                        <div class="flex flex-wrap gap-3 pb-4">
                            <label class="text-sm font-medium leading-normal flex items-center justify-center rounded border border-[#346534] px-4 h-11 text-white has-[:checked]:border-[3px] has-[:checked]:px-3.5 has-[:checked]:border-[#19e619] relative cursor-pointer">
                                { "CSV" }
                                <input checked={(*format).eq(&Format::Csv)} class="invisible absolute" name="format-select" type="radio" onclick={on_csv_click}/>
                            </label>
                            <label class="text-sm font-medium leading-normal flex items-center justify-center rounded border border-[#346534] px-4 h-11 text-white has-[:checked]:border-[3px] has-[:checked]:px-3.5 has-[:checked]:border-[#19e619] relative cursor-pointer">
                                { "JSON" }
                                <input checked={(*format).eq(&Format::Json)} class="invisible absolute" name="format-select" type="radio" onclick={on_json_click}/>
                            </label>
                        </div>
                        { if (*current_tab).eq(&Tabs::Import) {
                            html! {
                                <div class="flex flex-col py-4">
                                    <div class="flex flex-col items-center gap-6 rounded border-2 border-dashed border-[#346534] px-6 py-14">
                                        <span class="material-symbols-outlined text-5xl text-[#346534]">
                                            { "folder_open" }
                                        </span>
                                        <div class="flex max-w-[480px] flex-col items-center gap-2">
                                            <p class="text-white text-lg font-bold leading-tight tracking-[-0.015em] max-w-[480px] text-center">
                                                { "DRAG & DROP FILE HERE" }
                                            </p>
                                            <p class="text-[#93c893] text-sm font-normal leading-normal max-w-[480px] text-center">
                                                { "Maximum file size: 5MB" }
                                            </p>
                                        </div>
                                        <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded h-10 px-4 bg-[#244724] hover:bg-primary/30 text-white text-sm font-bold leading-normal tracking-[0.015em]">
                                            <span class="truncate">
                                                { "[ BROWSE FILES ]" }
                                            </span>
                                        </button>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                        // <div class="flex flex-col pt-6 flex-1">
                        //     <h3 class="text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-3">
                        //         { "> STATUS LOG:" }
                        //     </h3>
                        //     <div class="bg-black/30 rounded border border-[#346534] p-4 h-24 overflow-y-auto">
                        //         <p class="text-[#93c893] text-sm font-mono">
                        //             { "> FILE SELECTED: budget_data.csv" }
                        //         </p>
                        //         <p class="text-[#93c893] text-sm font-mono">
                        //             { "> PARSING..." }
                        //         </p>
                        //         <p class="text-primary text-sm font-mono">
                        //             { "> SUCCESS: DATA INTEGRATED." }
                        //         </p>
                        //     </div>
                        // </div>
                    </div>
                    <div class="flex flex-col sm:flex-row gap-4 pt-8">
                        <button
                            class="flex w-full sm:w-auto flex-1 cursor-pointer items-center justify-center overflow-hidden rounded h-12 px-6 bg-primary/90 hover:bg-primary text-background-dark text-base font-bold leading-normal tracking-[0.015em]"
                            onclick={on_initiate_click}
                        >
                            <span class="truncate">
                                {if (*current_tab).eq(&Tabs::Import) {
                                    "[ INITIATE IMPORT ]"
                                } else {
                                    "[ INITIATE EXPORT ]"
                                }}
                            </span>
                        </button>
                        <button
                            class="flex w-full sm:w-auto cursor-pointer items-center justify-center overflow-hidden rounded h-12 px-6 border border-[#346534] text-[#93c893] hover:border-primary hover:text-primary text-base font-bold leading-normal tracking-[0.015em]"
                            onclick={handle_close.clone()}
                        >
                            <span class="truncate">
                                { "< CANCEL >" }
                            </span>
                        </button>
                    </div>
                </div>
            </div>
        </Dialog>
    }
}
