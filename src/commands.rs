use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CommandsProps {
    pub on_add_expense_click: Callback<MouseEvent>,
    pub on_view_reports_click: Callback<MouseEvent>,
    pub on_manage_limits_click: Callback<MouseEvent>,
}

#[function_component]
pub fn Commands(props: &CommandsProps) -> Html {
    let CommandsProps {
        on_add_expense_click,
        on_view_reports_click,
        on_manage_limits_click,
    } = props;

    html! {
    <section>
        <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">{"// COMMANDS //"}</h3>
        <div class="p-4 flex flex-col sm:flex-row gap-4">
            <button class="flex-1 text-left p-4 border border-primary/30 rounded hover:bg-primary/20 hover:text-white transition-colors duration-200" onclick={on_add_expense_click}>
                <span class="font-bold">{">_ ADD EXPENSE"}</span>
            </button>
            <button class="flex-1 text-left p-4 border border-primary/30 rounded hover:bg-primary/20 hover:text-white transition-colors duration-200" onclick={on_view_reports_click}>
                <span class="font-bold">{">_ VIEW REPORTS"}</span>
            </button>
            <button class="flex-1 text-left p-4 border border-primary/30 rounded hover:bg-primary/20 hover:text-white transition-colors duration-200" onclick={on_manage_limits_click}>
                <span class="font-bold">{">_ MANAGE LIMITS"}</span>
            </button>
        </div>
    </section>
    }
}
