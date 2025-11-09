use crate::add_expense::AddExpense;
use crate::analysis::Analysis;
use crate::commands::Commands;
use crate::components::{footer::Footer, header::Header};
use crate::manage_limits::ManageLimits;
use crate::overview::Overview;
use crate::state::{Action, State, Transaction, KEY};
use crate::transaction_log::TransactionLog;
use crate::view_reports::ViewReports;
use chrono::Utc;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let add_expense_open = use_state(|| false);
    let view_reports_open = use_state(|| false);
    let manage_limits_open = use_state(|| false);

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

    let open_manage_limits = {
        let manage_limits_open = manage_limits_open.clone();
        Callback::from(move |_| manage_limits_open.set(true))
    };

    let on_close_manage_limits = {
        let manage_limits_open = manage_limits_open.clone();
        Callback::from(move |_| manage_limits_open.set(false))
    };

    let state = use_reducer(|| LocalStorage::get(KEY).unwrap_or_else(|_| State::default()));

    let on_submit_transaction = {
        let state = state.clone();
        let add_expense_open = add_expense_open.clone();

        Callback::from(move |(amount, description, category, notes)| {
            let date = Utc::now();
            state.dispatch(Action::AddTransaction(Transaction {
                date,
                amount,
                description,
                category,
                notes,
            }));
            add_expense_open.set(false);
        })
    };

    let on_submit_limits = {
        let state = state.clone();
        let manage_limits_open = manage_limits_open.clone();

        Callback::from(move |(monthly_limit, categories)| {
            state.dispatch(Action::EditLimits((monthly_limit, categories)));
            manage_limits_open.set(false);
        })
    };

    // Effect
    use_effect_with(state.clone(), |state| {
        LocalStorage::set(
            KEY,
            State {
                transactions: state.transactions.clone(),
                categories: state.categories.clone(),
                monthly_limit: state.monthly_limit,
            },
        )
        .expect("failed to set");
    });

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
        <AddExpense
            open={*add_expense_open}
            on_close={on_close_add_expense}
            categories={state.categories.clone()}
            on_submit={on_submit_transaction}
        />
        <ViewReports open={*view_reports_open} on_close={on_close_view_reports}/>
        <ManageLimits
            open={*manage_limits_open}
            on_close={on_close_manage_limits}
            categories={state.categories.clone()}
            monthly_limit={state.monthly_limit}
            on_submit={on_submit_limits}
        />
    </div>
    }
}
