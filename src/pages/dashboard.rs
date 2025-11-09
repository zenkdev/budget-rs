use crate::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DashboardProps {
    pub transactions: Vec<Transaction>,
    pub categories: Vec<Category>,
    pub monthly_limit: f64,
    pub open_add_expense: Callback<MouseEvent>,
    pub open_view_reports: Callback<MouseEvent>,
    pub open_manage_limits: Callback<MouseEvent>,
}

#[function_component]
pub fn Dashboard(props: &DashboardProps) -> Html {
    let DashboardProps {
        transactions,
        categories,
        monthly_limit,
        open_add_expense,
        open_view_reports,
        open_manage_limits,
    } = props;
    html! {
        <div class="layout-container flex h-full grow flex-col">
            <div class="px-4 sm:px-10 md:px-20 lg:px-40 flex flex-1 justify-center py-5">
                <div class="layout-content-container flex flex-col max-w-[960px] flex-1">
                    <Header/>
                    <main class="flex flex-col gap-4 mt-4">
                        <MonthlyOverview
                            transactions={transactions.clone()}
                            monthly_limit={monthly_limit}
                        />
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                            <CategoryAnalysis
                                transactions={transactions.clone()}
                                categories={categories.clone()}
                            />
                            <TransactionLogs transactions={transactions.clone()}/>
                        </div>
                        <Commands
                            on_add_expense_click={open_add_expense}
                            on_view_reports_click={open_view_reports}
                            on_manage_limits_click={open_manage_limits}
                        />
                    </main>
                    <Footer/>
                </div>
            </div>
        </div>
    }
}
