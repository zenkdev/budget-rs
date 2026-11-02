use crate::prelude::*;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use web_sys::{FocusEvent, HtmlInputElement, KeyboardEvent};

#[function_component]
pub fn AddTransaction() -> Html {
    let navigator = use_navigator().unwrap();
    let close_action = {
        let navigator = navigator.clone();
        Rc::new(move || {
            navigator.push(&Route::Home);
        })
    };

    let on_close = {
        let close_action = close_action.clone();
        Callback::from(move |_: MouseEvent| {
            close_action();
        })
    };

    let state = use_context::<State>().expect("no ctx found");
    let dispatch = use_context::<DispatchState>().expect("no ctx found");
    let state_for_blur = state.clone();
    let categories = state.categories;

    let edit_transaction = yew_router::hooks::use_location()
        .and_then(|location| parse_edit_id(location.query_str()))
        .and_then(|id| state.transactions.iter().find(|t| t.id == id).cloned());
    let edit_id = edit_transaction.as_ref().map(|t| t.id);

    let form = use_reducer(|| match edit_transaction {
        Some(t) => FormState {
            amount: t.amount,
            date: t.date,
            description: t.description,
            category: t.category,
            notes: t.notes,
        },
        None => FormState::default(),
    });

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

    let on_blur_description = {
        let form = form.clone();
        let state = state_for_blur;

        Callback::from(move |e: FocusEvent| {
            if form.category != 0 {
                return;
            }

            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            if let Some(category) = latest_category_for_description(&state.transactions, &value) {
                tracing::info!("on_blur_description: category {} for \"{}\"", category, value);
                form.dispatch(FormAction::Category(category));
            }
        })
    };

    let submit_action = {
        let form = form.clone();
        let dispatch = dispatch.clone();
        let navigator = navigator.clone();
        Rc::new(move || {
            if form.amount != 0.0 && !form.description.is_empty() && form.category != 0 {
                let transaction = Transaction {
                    id: edit_id.unwrap_or(0),
                    amount: form.amount,
                    date: form.date,
                    description: form.description.clone(),
                    category: form.category,
                    notes: form.notes.clone(),
                };
                match edit_id {
                    Some(id) => dispatch.emit(Action::EditTransaction(id, transaction)),
                    None => dispatch.emit(Action::AddTransaction(transaction)),
                }
                navigator.push(&Route::Home);
            }
        })
    };

    let on_submit = {
        let submit_action = submit_action.clone();
        Callback::from(move |_: MouseEvent| {
            submit_action();
        })
    };

    let on_keydown = {
        let submit_action = submit_action.clone();
        let close_action = close_action.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.ctrl_key() && e.key() == "Enter" {
                e.prevent_default();
                submit_action();
            } else if e.key() == "Escape" {
                e.prevent_default();
                close_action();
            }
        })
    };

    html! {
        <div class="layout-container flex h-full grow flex-col">
            <div class="flex flex-1 justify-center py-5">
                <div class="layout-content-container flex w-full max-w-3xl flex-col flex-1">
                    <div class="flex flex-wrap justify-center gap-3 p-4">
                        <p class="text-primary text-3xl md:text-4xl font-black leading-tight tracking-[-0.033em]">
                            { if edit_id.is_some() { html! { "[ EDIT TRANSACTION ENTRY ]" } } else { html! { "[ NEW TRANSACTION ENTRY ]" } } }
                        </p>
                    </div>
                    <form class="flex flex-col gap-6 mt-8" onkeydown={on_keydown}>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
                            <div class="flex flex-col">
                                <label class="text-base font-medium leading-normal pb-2" for="amount">{"AMOUNT:"}</label>
                                <InputAmount
                                    class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 placeholder:text-primary/50 p-4 text-base font-normal leading-normal"
                                    value={form.amount}
                                    onchange={on_change_amount}
                                    autofocus={true}
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
                                onblur={on_blur_description}
                                placeholder="Enter expense name..."
                            />
                        </div>
                        <div class="flex flex-col">
                            <label class="text-base font-medium leading-normal pb-2" for="category">{"CATEGORY:"}</label>
                            <select
                                class="form-select appearance-none w-full min-w-0 flex-1 resize-none overflow-hidden rounded-none text-primary focus:outline-0 focus:ring-0 border border-primary/30 bg-black/30 focus:border-primary h-14 p-4 text-base font-normal leading-normal"
                                id="category"
                                name="category"
                                onchange={on_change_category}
                            >
                                <option value="0" selected={form.category == 0} disabled={true}>{"Select category..."}</option>
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
                            />
                        </div>
                        <div class="flex flex-col sm:flex-row items-center justify-center gap-4 mt-8">
                            <Button
                                class="w-full sm:w-auto"
                                button_type="submit"
                                onclick={on_submit.clone()}
                                disabled={form.amount == 0.0 || form.description.is_empty() || form.category == 0}
                            >
                                { if edit_id.is_some() { html! { "[ UPDATE ENTRY ]" } } else { html! { "[ SAVE ENTRY ]" } } }
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

fn parse_edit_id(query: &str) -> Option<usize> {
    query
        .trim_start_matches('?')
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.split('=');
            match (parts.next(), parts.next()) {
                (Some("id"), Some(value)) => value.parse::<usize>().ok(),
                _ => None,
            }
        })
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
