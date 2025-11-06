use crate::add_expense::AddExpense;
use crate::analysis::Analysis;
use crate::commands::Commands;
use crate::footer::Footer;
use crate::header::Header;
use crate::manage_budgets::ManageBudgets;
use crate::overview::Overview;
use crate::transaction_log::TransactionLog;
use crate::view_reports::ViewReports;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let add_expense_open = use_state(|| false);
    let view_reports_open = use_state(|| false);
    let manage_budgets_open = use_state(|| false);

    let open_add_expense = {
        let add_expense_open = add_expense_open.clone();
        Callback::from(move |_| add_expense_open.set(true))
    };

    let on_close_add_expense = {
        let add_expense_open = add_expense_open.clone();
        Callback::from(move |_| add_expense_open.set(false))
    };

    let open_view_reports = {
        let view_reports_open = view_reports_open.clone();
        Callback::from(move |_| view_reports_open.set(true))
    };

    let on_close_view_reports = {
        let view_reports_open = view_reports_open.clone();
        Callback::from(move |_| view_reports_open.set(false))
    };

    let open_manage_budgets = {
        let manage_budgets_open = manage_budgets_open.clone();
        Callback::from(move |_| manage_budgets_open.set(true))
    };

    let on_close_manage_budgets = {
        let manage_budgets_open = manage_budgets_open.clone();
        Callback::from(move |_| manage_budgets_open.set(false))
    };

    html! {
    <div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden">
        <div class="scanlines"></div>
        <div class="layout-container flex h-full grow flex-col">
            <div class="px-4 sm:px-10 md:px-20 lg:px-40 flex flex-1 justify-center py-5">
                <div class="layout-content-container flex flex-col max-w-[960px] flex-1">
                    <Header/>
                    <main class="flex flex-col gap-4 mt-4">
                        <Overview/>
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                            <Analysis/>
                            <TransactionLog/>
                        </div>
                        <Commands on_add_expense_click={open_add_expense} on_view_reports_click={open_view_reports} on_manage_budgets_click={open_manage_budgets}/>
                    </main>
                    <Footer/>
                </div>
            </div>
        </div>
        <AddExpense open={*add_expense_open} on_close={on_close_add_expense}/>
        <ViewReports open={*view_reports_open} on_close={on_close_view_reports}/>
        <ManageBudgets open={*manage_budgets_open} on_close={on_close_manage_budgets}/>
    </div>
    }
}
