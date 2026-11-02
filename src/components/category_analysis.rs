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
            <div class="p-4 border border-primary/30 rounded flex-1 overflow-x-auto">
                <table class="w-full table-fixed text-left text-sm">
                    <thead>
                        <tr>
                            <th class="py-2 pr-4 font-medium text-primary/80">{"CATEGORY"}</th>
                            <th class="py-2 pr-4 w-32 text-right font-medium text-primary/80">{"AMOUNT"}</th>
                            <th class="py-2 w-52"></th>
                        </tr>
                    </thead>
                    <tbody>
                        {for categories.iter().map(|cat| {
                            let spent = get_category_spent_this_month(cat.id, &transactions);
                            html!{
                                <tr key={cat.id.to_string()}>
                                    <td class="py-1.5 pr-4 truncate">{ cat.name.clone() }</td>
                                    <td class="py-1.5 pr-4 text-right">{ fmt_amount(spent) }</td>
                                    <td class="py-1.5">{ progress_bar(get_percent(spent, cat.limit)) }</td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        </section>
    }
}
