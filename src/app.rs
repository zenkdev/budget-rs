use crate::prelude::*;
use chrono::Utc;
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

    let state = use_app_state();

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

    html! {
        <div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden">
            <div class="scanlines"></div>
            <Dashboard
                open_add_expense={open_add_expense}
                open_view_reports={open_view_reports}
                open_manage_limits={open_manage_limits}
            />
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
