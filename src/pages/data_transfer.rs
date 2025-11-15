use crate::prelude::*;
use web_sys::{File, HtmlInputElement};

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

#[function_component]
pub fn DataTransfer() -> Html {
    let navigator = use_navigator().unwrap();
    let on_close = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
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

    let file_ref = use_node_ref();
    let on_browse_files_click = {
        let file_ref = file_ref.clone();
        Callback::from(move |_| file_ref.cast::<HtmlInputElement>().unwrap().click())
    };

    let selected_file = use_state(|| Option::<File>::None);
    let on_change_file = {
        let selected_file = selected_file.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file = input.files().unwrap().get(0).unwrap();
            if file.type_() == "text/csv" {
                selected_file.set(Some(file));
            } else {
                selected_file.set(None);
            }
        })
    };

    let drop_target_ref = use_node_ref();
    let drop_target_state = use_drop(drop_target_ref.clone());
    use_effect_with(
        (selected_file.clone(), drop_target_state.files.clone()),
        |(selected_file, files)| {
            let selected_file = selected_file.clone();
            if let Some(files) = files.as_ref() {
                let file = files.get(0).unwrap();
                selected_file.set(Some(file.clone()));
            }
        },
    );

    let state = use_context::<State>().expect("no ctx found");
    let dispatch = use_context::<DispatchState>().expect("no ctx found");
    let on_initiate_click = {
        if (*current_tab).eq(&Tabs::Import) {
            let selected_file = selected_file.clone();
            let dispatch = dispatch.clone();
            Callback::from(move |_| {
                if let Some(file) = &*selected_file {
                    let dispatch = dispatch.clone();
                    let navigator = navigator.clone();
                    parse_csv_file(
                        file.clone(),
                        Callback::from(move |state: State| {
                            dispatch.emit(Action::Load(state));
                            navigator.push(&Route::Home)
                        }),
                    );
                }
            })
        } else {
            let transactions = state.transactions.clone();
            let categories = state.categories.clone();
            let monthly_limit = state.monthly_limit;
            let navigator = navigator.clone();
            Callback::from(move |_| {
                save_data_as_csv_file(transactions.clone(), categories.clone(), monthly_limit);
                navigator.push(&Route::Home)
            })
        }
    };

    html! {
        <div class="mx-auto flex my-8 w-full max-w-[960px] grow flex-col border-2 border-primary/20 bg-[#112211] rounded-lg">
            <div class="flex flex-col flex-1 p-4 md:p-6 lg:p-8">
                <div class="flex flex-wrap justify-between gap-3 items-start">
                    <div class="flex min-w-72 flex-col gap-3">
                        <p class="text-primary text-4xl font-black leading-tight tracking-[-0.033em]">
                            { "[ DATA TRANSFERENCE INTERFACE ]" }
                        </p>
                    </div>
                    <button class="flex items-center justify-center text-[#93c893] hover:text-primary transition-colors" onclick={on_close.clone()}>
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
                                { "[ INBOUND DATA STREAM ]" }
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
                                { "[ OUTBOUND DATA STREAM ]" }
                            </p>
                        </button>
                    </div>
                </div>
                <div class="flex flex-col flex-1">
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
                            <input checked={(*format).eq(&Format::Json)} class="invisible absolute" name="format-select" type="radio" onclick={on_json_click} disabled={true} />
                        </label>
                    </div>
                    { if (*current_tab).eq(&Tabs::Import) {
                        html! {
                            <div class="flex flex-col py-4">
                                <div ref={drop_target_ref} class="flex flex-col items-center gap-6 rounded border-2 border-dashed border-[#346534] px-6 py-14 hover:border-[#93c893]">
                                    <span class="material-symbols-outlined !text-5xl text-[#346534]">
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
                                    <button
                                        class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded h-10 px-4 bg-[#244724] hover:bg-primary/30 text-white text-sm font-bold leading-normal tracking-[0.015em]"
                                        onclick={on_browse_files_click}>
                                        <span class="truncate">
                                            { "[ BROWSE FILES ]" }
                                        </span>
                                    </button>
                                    <input
                                        ref={file_ref}
                                        type="file"
                                        class="invisible absolute"
                                        multiple={false}
                                        accept={(*format).eq(&Format::Json).then_some(".json").or(Some(".csv"))}
                                        onchange={on_change_file}
                                    />
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                    <div class="flex flex-col pt-6 flex-1">
                        <h3 class="text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-3">
                            { "> STATUS LOG:" }
                        </h3>
                        <div class="bg-black/30 rounded border border-[#346534] p-4 h-24 overflow-y-auto">
                            { if let Some(file) = &*selected_file {
                                html! {
                                    <p class="text-[#93c893] text-sm font-mono">
                                        { format!("> FILE SELECTED: {}", file.name()) }
                                    </p>
                                }
                            } else {
                                html! {}
                            }}
                            // <p class="text-[#93c893] text-sm font-mono">
                            //     { "> PARSING..." }
                            // </p>
                            // <p class="text-primary text-sm font-mono">
                            //     { "> SUCCESS: DATA INTEGRATED." }
                            // </p>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col sm:flex-row gap-4 pt-8">
                    <Button
                        class="w-full sm:w-auto flex-1"
                        onclick={on_initiate_click}
                    >
                        <span class="truncate">
                            {if (*current_tab).eq(&Tabs::Import) {
                                "[ INITIATE IMPORT ]"
                            } else {
                                "[ INITIATE EXPORT ]"
                            }}
                        </span>
                    </Button>
                    <Button
                        class="w-full sm:w-auto"
                        variant={ButtonVariant::Cancel}
                        onclick={on_close.clone()}
                    >
                        <span class="truncate">
                            { "< CANCEL >" }
                        </span>
                    </Button>
                </div>
                <div class="pt-8">
                    <p class="text-primary text-base font-normal leading-normal">
                        { "STATUS: AWAITING COMMAND" }
                        <span class="blinking-cursor">{ "_" }</span>
                    </p>
                </div>
            </div>
        </div>
    }
}
