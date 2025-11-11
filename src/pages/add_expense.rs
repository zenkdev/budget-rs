use crate::prelude::*;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[function_component]
pub fn AddExpense() -> Html {
    let navigator = use_navigator().unwrap();
    let on_close = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div class="layout-container flex h-full grow flex-col">
            <div class="flex flex-1 justify-center py-5">
                <div class="layout-content-container flex w-full max-w-3xl flex-col flex-1">
                    <div class="flex flex-wrap justify-center gap-3 p-4">
                        <p class="text-primary text-3xl md:text-4xl font-black leading-tight tracking-[-0.033em]">
                            { "[ < NEW EXPENSE LOG > ]" }
                        </p>
                    </div>
                    <AddExpenseForm on_close={on_close.clone()} />
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct FormState {
    amount: f64,
    date: DateTime<Local>,
    description: String,
    category: usize,
    notes: String,
}

impl Eq for FormState {}

enum FormAction {
    Amount(f64),
    Date(DateTime<Local>),
    Description(String),
    Category(usize),
    Notes(String),
}

impl Reducible for FormState {
    type Action = FormAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            FormAction::Amount(amount) => FormState {
                amount,
                date: self.date,
                description: self.description.clone(),
                category: self.category,
                notes: self.notes.clone(),
            }
            .into(),
            FormAction::Date(date) => FormState {
                amount: self.amount,
                date,
                description: self.description.clone(),
                category: self.category,
                notes: self.notes.clone(),
            }
            .into(),
            FormAction::Description(description) => FormState {
                amount: self.amount,
                date: self.date,
                description,
                category: self.category,
                notes: self.notes.clone(),
            }
            .into(),
            FormAction::Category(category) => FormState {
                amount: self.amount,
                date: self.date,
                description: self.description.clone(),
                category,
                notes: self.notes.clone(),
            }
            .into(),
            FormAction::Notes(notes) => FormState {
                amount: self.amount,
                date: self.date,
                description: self.description.clone(),
                category: self.category,
                notes: notes.clone(),
            }
            .into(),
        }
    }
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            amount: 0.0,
            date: Local::now(),
            description: String::new(),
            category: 0,
            notes: String::new(),
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct AddExpenseFormProps {
    pub on_close: Callback<()>,
}

#[function_component]
pub fn AddExpenseForm(props: &AddExpenseFormProps) -> Html {
    let AddExpenseFormProps { on_close } = props;
    let app_state = use_context::<State>().expect("no ctx found");
    let categories = app_state.categories;

    let state = use_reducer(FormState::default);

    let on_change_amount = {
        let state = state.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_amount(&e);
            tracing::info!("on_change_amount: {}", value);
            state.dispatch(FormAction::Amount(value));
        })
    };

    let on_change_description = {
        let state = state.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_string(&e);
            tracing::info!("on_change_description: {}", value);
            state.dispatch(FormAction::Description(value));
        })
    };

    let on_change_category = {
        let state = state.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_usize(&e);
            tracing::info!("on_change_category: {}", value);
            state.dispatch(FormAction::Category(value));
        })
    };

    let on_change_notes = {
        let state = state.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_string(&e);
            tracing::info!("on_change_notes: {}", value);
            state.dispatch(FormAction::Notes(value));
        })
    };

    let on_cancel = {
        let on_close = on_close.clone();

        Callback::from(move |_| on_close.emit(()))
    };

    let dispatch = use_context::<DispatchState>().expect("no ctx found");
    let on_submit = {
        let amount = state.amount;
        let description = state.description.clone();
        let category = state.category;
        let notes = state.notes.clone();
        let dispatch = dispatch.clone();
        let on_close = on_close.clone();

        Callback::from(move |_| {
            let date = Local::now();
            dispatch.emit(Action::AddTransaction(Transaction {
                date,
                amount,
                description: description.clone(),
                category,
                notes: notes.clone(),
            }));
            on_close.emit(());
        })
    };

    html! {
        <form class="flex flex-col gap-6 mt-8">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
                <div class="flex flex-col">
                    <label class="text-base font-medium leading-normal pb-2" for="amount">{"AMOUNT:"}</label>
                    <input
                        class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal"
                        id="amount"
                        name="amount"
                        // placeholder="&gt; $ [___]"
                        value={fmt_amount(state.amount)}
                        onchange={on_change_amount}
                    />
                </div>
                <div class="flex flex-col">
                    <label class="text-base font-medium leading-normal pb-2" for="date">{"DATE:"}</label>
                    <input
                        class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal"
                        id="date"
                        name="date"
                        value={state.date.format("%Y-%m-%d").to_string()}
                        // onchange={on_change_date}
                        placeholder="YYYY-MM-DD"
                        type="date"
                        readonly={true}
                    />
                </div>
            </div>
            <div class="flex flex-col">
                <label class="text-base font-medium leading-normal pb-2" for="description">{"DESCRIPTION:"}</label>
                <input
                    class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal"
                    id="description"
                    name="description"
                    value={state.description.clone()}
                    onchange={on_change_description}
                    placeholder="Enter expense name..."
                />
            </div>
            <div class="flex flex-col">
                <label class="text-base font-medium leading-normal pb-2" for="category">{"CATEGORY:"}</label>
                <select
                    class="form-select appearance-none w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 p-4 text-base font-normal leading-normal bg-no-repeat bg-right"
                    id="category"
                    name="category"
                    onchange={on_change_category}
                    style="background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%2339ff14' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.5rem center; background-size: 1.5em 1.5em;"
                >
                    <option value="0" selected={state.category == 0}>{"Select category..."}</option>
                    {for categories.iter().cloned().map(|category| html!{
                        <option value={category.id.to_string()} selected={state.category == category.id}>{category.name.clone()}</option>
                    })}
                </select>
            </div>
            <div class="flex flex-col">
                <label class="text-base font-medium leading-normal pb-2" for="notes">{"NOTES:"}</label>
                <textarea
                    class="form-textarea flex w-full min-w-0 flex-1 resize-y overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-32 placeholder:text-primary/50 p-4 text-base font-normal leading-normal" id="notes"
                    name="notes"
                    value={state.notes.clone()}
                    onchange={on_change_notes}
                    placeholder="Add optional details here..."
                ></textarea>
            </div>
            <div class="flex flex-col sm:flex-row items-center justify-center gap-4 mt-8">
                <button
                    class="w-full sm:w-auto px-8 py-3 bg-transparent border border-primary text-primary font-bold hover:bg-primary/20 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 focus:ring-offset-background-dark transition-colors duration-200"
                    type="submit"
                    onclick={on_submit}
                    disabled={state.amount == 0.0 || state.description.is_empty() || state.category == 0}
                >
                    {"[ SAVE ENTRY ]"}
                </button>
                <button
                    class="w-full sm:w-auto px-8 py-3 text-primary/70 font-normal hover:text-primary focus:outline-none focus:ring-2 focus:ring-primary/50 focus:ring-offset-2 focus:ring-offset-background-dark transition-colors duration-200"
                    type="button"
                    onclick={on_cancel}
                >
                    {"[ CANCEL ]"}
                </button>
            </div>
        </form>
    }
}
