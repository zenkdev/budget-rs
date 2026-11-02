use crate::prelude::*;
use std::collections::BTreeMap;

#[function_component]
pub fn TransactionLogs() -> Html {
    let navigator = use_navigator().unwrap();
    let state = use_context::<State>().expect("no ctx found");
    let mut transactions = state.transactions.clone();

    transactions.sort_by_key(|t| t.date);
    transactions.reverse();
    transactions.truncate(10);

    let on_edit = {
        let navigator = navigator.clone();
        Callback::from(move |id: usize| {
            let mut query = BTreeMap::new();
            query.insert("id", id.to_string());
            navigator
                .push_with_query(&Route::AddTransaction, &query)
                .unwrap();
        })
    };

    html! {
    <section class="flex flex-col">
        <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">
            { "// TRANSACTION LOGS //" }
        </h3>
        <div class="h-4"></div>
        <div class="p-4 border border-primary/30 rounded flex-1 overflow-x-auto">
            <table class="w-full text-left text-sm">
                <thead>
                    <tr>
                        <th class="py-2 pr-4 font-medium text-primary/80">{"DATE"}</th>
                        <th class="py-2 pr-4 font-medium text-primary/80">{"DESCRIPTION"}</th>
                        <th class="py-2 pr-4 text-right font-medium text-primary/80">{"AMOUNT"}</th>
                        <th class="py-2 text-right"></th>
                    </tr>
                </thead>
                <tbody>
                    {for transactions.iter().map(|t| {
                        let id = t.id;
                        html!{
                            <tr key={id.to_string()}>
                                <td class="py-1.5 pr-4 whitespace-nowrap">{ fmt_date(t.date) }</td>
                                <td class="py-1.5 pr-4">{ t.description.clone() }</td>
                                <td class="py-1.5 pr-4 text-right">{ fmt_amount(t.amount)}</td>
                                <td class="py-1.5 text-right">
                                    <button
                                        class="text-primary/60 hover:text-primary text-xs font-bold tracking-wide cursor-pointer transition-colors duration-200"
                                        onclick={on_edit.reform(move |_| id)}
                                    >
                                        { "[EDIT]" }
                                    </button>
                                </td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    </section>
    }
}
