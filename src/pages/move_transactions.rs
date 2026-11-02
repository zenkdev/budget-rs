use crate::prelude::*;
use web_sys::Event;

#[function_component]
pub fn MoveTransactions() -> Html {
    let state = use_context::<State>().expect("no ctx found");
    let categories = state.categories;
    let transactions = state.transactions;

    let navigator = use_navigator().unwrap();
    let dispatch = use_context::<DispatchState>().expect("no ctx found");

    let from = use_state(|| 0usize);
    let to = use_state(|| 0usize);

    let on_change_from = {
        let from = from.clone();

        Callback::from(move |e: Event| {
            from.set(target_input_value_usize(&e));
        })
    };

    let on_change_to = {
        let to = to.clone();

        Callback::from(move |e: Event| {
            to.set(target_input_value_usize(&e));
        })
    };

    let count = if *from == 0 {
        0
    } else {
        transactions
            .iter()
            .filter(|transaction| transaction.category == *from)
            .count()
    };

    let valid = *from != 0 && *to != 0 && *from != *to && count > 0;

    let status = if *from == 0 {
        "> SELECT SOURCE CATEGORY.".to_string()
    } else if *to == 0 {
        format!("> {} TRANSACTIONS FOUND. SELECT TARGET CATEGORY.", count)
    } else if *from == *to {
        "> CATEGORIES MUST DIFFER.".to_string()
    } else if count == 0 {
        "> SOURCE CATEGORY HAS NO TRANSACTIONS.".to_string()
    } else {
        format!(
            "> {} TRANSACTIONS WILL BE MOVED.",
            count
        )
    };

    let on_submit = {
        let dispatch = dispatch.clone();
        let navigator = navigator.clone();
        let from = from.clone();
        let to = to.clone();

        Callback::from(move |_| {
            if !valid {
                return;
            }

            tracing::info!("move_transactions: {} -> {}", *from, *to);
            dispatch.emit(Action::MoveCategoryTransactions((*from, *to)));
            navigator.push(&Route::Home);
        })
    };

    html! {
        <div class="layout-container flex h-full grow flex-col">
            <div class="px-4 sm:px-10 md:px-20 lg:px-40 flex flex-1 justify-center py-5">
                <div class="layout-content-container flex flex-col max-w-[960px] flex-1">
                    <div class="flex px-4 py-3 justify-start">
                        <HomeLink variant={HomeLinkVariant::Back} />
                    </div>
                    <div class="flex flex-wrap justify-between gap-3 p-4">
                        <div class="flex min-w-72 flex-col gap-3">
                            <p class="text-primary text-4xl font-black leading-tight tracking-[-0.033em]">
                                { "[ MOVE TRANSACTIONS ]" }
                            </p>
                            <p class="text-[#93c893] text-base font-normal leading-normal">
                                { "> REASSIGN TRANSACTIONS FROM ONE CATEGORY TO ANOTHER." }
                            </p>
                        </div>
                    </div>
                    <div class="flex max-w-[480px] flex-col gap-6 px-4 py-3">
                        <label class="flex flex-col">
                            <p class="text-primary text-base font-medium leading-normal pb-2">{"> FROM: "}</p>
                            <select
                                class="form-select appearance-none w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 p-4 text-base font-normal leading-normal"
                                onchange={on_change_from}
                            >
                                <option value="0" selected={*from == 0} disabled={true}>{"Select category..."}</option>
                                {for categories.iter().filter(|category| category.id != *to).map(|category| html! {
                                    <option value={category.id.to_string()} selected={*from == category.id}>{category.name.clone()}</option>
                                })}
                            </select>
                        </label>
                        <label class="flex flex-col">
                            <p class="text-primary text-base font-medium leading-normal pb-2">{"> TO: "}</p>
                            <select
                                class="form-select appearance-none w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 p-4 text-base font-normal leading-normal"
                                onchange={on_change_to}
                            >
                                <option value="0" selected={*to == 0} disabled={true}>{"Select category..."}</option>
                                {for categories.iter().filter(|category| category.id != *from).map(|category| html! {
                                    <option value={category.id.to_string()} selected={*to == category.id}>{category.name.clone()}</option>
                                })}
                            </select>
                        </label>
                        <p class="text-[#93c893] text-base font-normal leading-normal">
                            { status }
                            <span class="blinking-cursor">{ "_" }</span>
                        </p>
                        <Button
                            class="min-w-[84px] max-w-[480px]"
                            disabled={!valid}
                            onclick={on_submit}>
                            <span class="truncate">
                                { "[ MOVE TRANSACTIONS ]" }
                            </span>
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    }
}
