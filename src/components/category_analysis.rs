use crate::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CategoryAnalysisProps {
    pub transactions: Vec<Transaction>,
    pub categories: Vec<Category>,
}

#[function_component]
pub fn CategoryAnalysis(props: &CategoryAnalysisProps) -> Html {
    let CategoryAnalysisProps {
        transactions,
        categories,
    } = props;

    let mut max_length = 0;
    for cat in categories {
        if cat.name.len() > max_length {
            max_length = cat.name.len();
        }
    }
    max_length += 3;

    html! {
        <section class="flex flex-col">
            <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">
                { "// CATEGORY ANALYSIS //" }
            </h3>
            <div class="p-4 border border-primary/30 rounded flex-1">
                { for categories.iter().cloned().map(|cat| html!{
                    <div class="flex justify-between gap-x-6 py-2">
                        <p class="text-primary/90 text-sm font-normal leading-normal">
                            { format!("> {}", pad_right(&cat.name, max_length, '.')) }
                        </p>
                        <p class="text-primary text-sm font-normal leading-normal text-right">
                            { fmt_amount(get_category_spent(cat.id, transactions)) }
                        </p>
                    </div>
                })}
            </div>
        </section>
    }
}
