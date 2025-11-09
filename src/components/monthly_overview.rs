use crate::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MonthlyOverviewProps {
    pub transactions: Vec<Transaction>,
    pub monthly_limit: f64,
}

#[function_component]
pub fn MonthlyOverview(props: &MonthlyOverviewProps) -> Html {
    let MonthlyOverviewProps {
        transactions,
        monthly_limit,
    } = props;

    let expendeture = transactions.iter().fold(0.0, |acc, t| acc + t.amount);
    let funds_remaining = *monthly_limit - expendeture;

    html! {
        <section>
            <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">
                { "// MONTHLY FINANCIAL OVERVIEW //" }
            </h3>
            <div class="flex flex-col sm:flex-row flex-wrap gap-4 p-4">
                <div class="flex min-w-[158px] flex-1 flex-col gap-2 rounded p-6 border border-primary/30">
                    <p class="text-primary/80 text-base font-medium leading-normal">
                        { "Monthly Limit" }
                    </p>
                    <p class="text-primary tracking-light text-2xl font-bold leading-tight text-glow">
                        { fmt_amount(monthly_limit.clone()) }
                    </p>
                </div>
                <div class="flex min-w-[158px] flex-1 flex-col gap-2 rounded p-6 border border-primary/30">
                    <p class="text-primary/80 text-base font-medium leading-normal">
                        { "Expenditure" }
                    </p>
                    <p class="text-primary tracking-light text-2xl font-bold leading-tight text-glow">
                        { fmt_amount(expendeture) }
                    </p>
                </div>
                <div class="flex min-w-[158px] flex-1 flex-col gap-2 rounded p-6 border border-primary/30">
                    <p class="text-primary/80 text-base font-medium leading-normal">
                        { "Funds Remaining" }
                    </p>
                    <p class="text-primary tracking-light text-2xl font-bold leading-tight text-glow">
                        { fmt_amount(funds_remaining) }
                    </p>
                </div>
            </div>
        </section>
    }
}
