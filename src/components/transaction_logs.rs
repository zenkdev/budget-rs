use crate::prelude::*;

#[function_component]
pub fn TransactionLogs() -> Html {
    let state = use_context::<State>().expect("no ctx found");
    let mut transactions = state.transactions.clone();

    transactions.sort_by_key(|t| t.date);
    transactions.reverse();
    transactions.truncate(10);

    html! {
    <section class="flex flex-col">
        <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">
            { "// TRANSACTION LOGS //" }
        </h3>
        <div class="p-4 border border-primary/30 rounded flex-1 overflow-x-auto">
            <table class="w-full text-left text-sm">
                <thead>
                    <tr>
                        <th class="py-2 pr-4 font-medium text-primary/80">{"DATE"}</th>
                        <th class="py-2 pr-4 font-medium text-primary/80">{"DESCRIPTION"}</th>
                        <th class="py-2 text-right font-medium text-primary/80">{"AMOUNT"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for transactions.iter().map(|t| html!{
                        <tr>
                            <td class="py-1.5 pr-4 whitespace-nowrap">{ fmt_date(t.date) }</td>
                            <td class="py-1.5 pr-4">{ t.description.clone() }</td>
                            <td class="py-1.5 text-right">{ fmt_amount(t.amount)}</td>
                        </tr>
                    })}
                </tbody>
            </table>
        </div>
    </section>
    }
}
