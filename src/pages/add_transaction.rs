use crate::prelude::*;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[function_component]
pub fn AddTransaction() -> Html {
    let navigator = use_navigator().unwrap();
    let on_close = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    let state = use_context::<State>().expect("no ctx found");
    let dispatch = use_context::<DispatchState>().expect("no ctx found");
    let categories = state.categories;

    let form = use_reducer(FormState::default);

    let on_change_amount = {
        let form = form.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_amount(&e);
            tracing::info!("on_change_amount: {}", value);
            form.dispatch(FormAction::Amount(value));
        })
    };

    let on_change_date = {
        let form = form.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_date(&e);
            tracing::info!("on_change_date: {}", value);
            form.dispatch(FormAction::Date(value));
        })
    };

    let on_change_description = {
        let form = form.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_string(&e);
            tracing::info!("on_change_description: {}", value);
            form.dispatch(FormAction::Description(value));
        })
    };

    let on_change_category = {
        let form = form.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_usize(&e);
            tracing::info!("on_change_category: {}", value);
            form.dispatch(FormAction::Category(value));
        })
    };

    let on_change_notes = {
        let form = form.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_string(&e);
            tracing::info!("on_change_notes: {}", value);
            form.dispatch(FormAction::Notes(value));
        })
    };

    let on_submit = {
        let amount = form.amount;
        let description = form.description.clone();
        let category = form.category;
        let notes = form.notes.clone();
        let dispatch = dispatch.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let date = Local::now();
            dispatch.emit(Action::AddTransaction(Transaction {
                date,
                amount,
                description: description.clone(),
                category,
                notes: notes.clone(),
            }));
            navigator.push(&Route::Home);
        })
    };

    html! {
        <div class="layout-container flex h-full grow flex-col">
            <div class="flex flex-1 justify-center py-5">
                <div class="layout-content-container flex w-full max-w-3xl flex-col flex-1">
                    <div class="flex flex-wrap justify-center gap-3 p-4">
                        <p class="text-primary text-3xl md:text-4xl font-black leading-tight tracking-[-0.033em]">
                            { "[ ADD NEW TRANSACTION ]" }
                        </p>
                    </div>
                    <form class="flex flex-col gap-6 mt-8">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
                            <div class="flex flex-col">
                                <label class="text-base font-medium leading-normal pb-2" for="amount">{"AMOUNT:"}</label>
                                <input
                                    class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal"
                                    id="amount"
                                    name="amount"
                                    value={fmt_amount(form.amount)}
                                    onchange={on_change_amount}
                                />
                            </div>
                            <div class="flex flex-col">
                                <label class="text-base font-medium leading-normal pb-2" for="date">{"DATE:"}</label>
                                <input
                                    class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal"
                                    id="date"
                                    name="date"
                                    value={form.date.format("%Y-%m-%d").to_string()}
                                    onchange={on_change_date}
                                    type="date"
                                />
                            </div>
                        </div>
                        <div class="flex flex-col">
                            <label class="text-base font-medium leading-normal pb-2" for="description">{"DESCRIPTION:"}</label>
                            <input
                                class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal"
                                id="description"
                                name="description"
                                value={form.description.clone()}
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
                                <option value="0" selected={form.category == 0}>{"Select category..."}</option>
                                {for categories.iter().cloned().map(|category| html!{
                                    <option value={category.id.to_string()} selected={form.category == category.id}>{category.name.clone()}</option>
                                })}
                            </select>
                        </div>
                        <div class="flex flex-col">
                            <label class="text-base font-medium leading-normal pb-2" for="notes">{"NOTES:"}</label>
                            <textarea
                                class="form-textarea flex w-full min-w-0 flex-1 resize-y overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-32 placeholder:text-primary/50 p-4 text-base font-normal leading-normal" id="notes"
                                name="notes"
                                value={form.notes.clone()}
                                onchange={on_change_notes}
                                placeholder="Add optional details here..."
                            ></textarea>
                        </div>
                        <div class="flex flex-col sm:flex-row items-center justify-center gap-4 mt-8">
                            <Button
                                class="w-full sm:w-auto"
                                button_type="submit"
                                onclick={on_submit}
                                disabled={form.amount == 0.0 || form.description.is_empty() || form.category == 0}
                            >
                                { "[ SAVE ENTRY ]" }
                            </Button>
                            <Button
                                class="w-full sm:w-auto"
                                variant={ButtonVariant::Cancel}
                                button_type="button"
                                onclick={on_close.clone()}
                            >
                                { "< CANCEL >" }
                            </Button>
                        </div>
                    </form>
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
