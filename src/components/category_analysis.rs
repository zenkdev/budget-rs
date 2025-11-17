use crate::prelude::*;

#[function_component]
pub fn CategoryAnalysis() -> Html {
    let state = use_context::<State>().expect("no ctx found");
    let transactions = state.transactions;
    let categories = state.categories;

    html! {
        <section class="flex flex-col">
            <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">
                { "// CATEGORY ANALYSIS //" }
            </h3>
            <div class="h-4"></div>
            <div class="p-4 border border-primary/30 rounded flex-1">
                { for categories.iter().map(|cat| html!{
                    <div class="grid grid-cols-3 justify-between gap-x-6 py-2">
                        <p class="text-primary/90 text-sm font-normal leading-normal dotted whitespace-nowrap overflow-hidden col-span-2">
                            { format!("> {}", cat.name.clone()) }
                        </p>
                        <p class="text-primary text-sm font-normal leading-normal text-right">
                            { fmt_amount(get_category_spent_this_month(cat.id, &transactions)) }
                        </p>
                    </div>
                })}
            </div>
        </section>
    }
}
