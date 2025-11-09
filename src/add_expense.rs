use crate::state::Category;
use chrono::{DateTime, Utc};
use numfmt::{Formatter, Precision};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use web_sys::{HtmlDialogElement, HtmlInputElement};
use yew::prelude::*;

fn fmt_currency(amount: f64) -> String {
    let f = Formatter::new() // start with blank representation
        .separator(',')
        .unwrap()
        .prefix("$")
        .unwrap()
        .precision(Precision::Decimals(2));
    f.fmt_string(amount)
}

fn target_input_value_string(e: &Event) -> String {
    let input: HtmlInputElement = e.target_unchecked_into();
    input.value()
}

fn target_input_value_f64(e: &Event) -> f64 {
    let input: HtmlInputElement = e.target_unchecked_into();
    input
        .value()
        .replace("$", "")
        .replace(",", "")
        .parse::<f64>()
        .unwrap_or(0.0)
}

fn target_input_value_usize(e: &Event) -> usize {
    let input: HtmlInputElement = e.target_unchecked_into();
    input.value().parse::<usize>().unwrap_or(0)
}

#[derive(PartialEq, Properties)]
pub struct AddExpenseProps {
    pub open: bool,
    pub on_close: Callback<MouseEvent>,
    pub categories: Vec<Category>,
    pub on_submit: Callback<(f64, String, usize, String)>,
}

#[function_component]
pub fn AddExpense(props: &AddExpenseProps) -> Html {
    let AddExpenseProps {
        open,
        on_close,
        categories,
        on_submit,
    } = props;
    let dialog_ref = use_node_ref();
    let prev = use_mut_ref(|| *open);

    {
        let dialog_ref = dialog_ref.clone();
        let open = open.clone();

        use_effect_with((dialog_ref, open, prev), |(dialog_ref, open, prev)| {
            let dialog = dialog_ref
                .cast::<HtmlDialogElement>()
                .expect("dialog_ref not attached to dialog element");

            tracing::info!("open: {}, prev: {}", *open, *prev.borrow_mut());
            if *prev.borrow_mut() != *open {
                *prev.borrow_mut() = *open;
                if *open {
                    dialog.show_modal().unwrap();
                } else {
                    dialog.close();
                }
            }
        });
    }

    html! {
    <dialog ref={dialog_ref} class="bg-background-dark font-display min-h-screen w-full p-4 sm:p-6 md:p-8 text-primary">
        { if *open {
            html! {
            <div class="layout-container flex h-full grow flex-col">
                <div class="flex flex-1 justify-center py-5">
                    <div class="layout-content-container flex w-full max-w-3xl flex-col flex-1">
                        <div class="flex flex-wrap justify-center gap-3 p-4">
                            <p class="text-primary text-3xl md:text-4xl font-black leading-tight tracking-[-0.033em]">{"[ < NEW EXPENSE LOG > ]"}</p>
                        </div>
                        <AddExpenseForm categories={categories.clone()} on_submit={on_submit.clone()} on_cancel={on_close.clone()} />
                    </div>
                </div>
            </div>
            }
        } else {
            html! {}
        }}
    </dialog>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct FormState {
    amount: f64,
    date: DateTime<Utc>,
    description: String,
    category: usize,
    notes: String,
}

impl Eq for FormState {}

enum FormAction {
    EditAmount(f64),
    EditDate(DateTime<Utc>),
    EditDescription(String),
    EditCategory(usize),
    EditNotes(String),
}

impl Reducible for FormState {
    type Action = FormAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            FormAction::EditAmount(amount) => FormState {
                amount,
                date: self.date,
                description: self.description.clone(),
                category: self.category,
                notes: self.notes.clone(),
            }
            .into(),
            FormAction::EditDate(date) => FormState {
                amount: self.amount,
                date,
                description: self.description.clone(),
                category: self.category,
                notes: self.notes.clone(),
            }
            .into(),
            FormAction::EditDescription(description) => FormState {
                amount: self.amount,
                date: self.date,
                description,
                category: self.category,
                notes: self.notes.clone(),
            }
            .into(),
            FormAction::EditCategory(category) => FormState {
                amount: self.amount,
                date: self.date,
                description: self.description.clone(),
                category,
                notes: self.notes.clone(),
            }
            .into(),
            FormAction::EditNotes(notes) => FormState {
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
            date: Utc::now(),
            description: String::new(),
            category: 0,
            notes: String::new(),
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct AddExpenseFormProps {
    pub categories: Vec<Category>,
    pub on_submit: Callback<(f64, String, usize, String)>,
    pub on_cancel: Callback<MouseEvent>,
}

#[function_component]
pub fn AddExpenseForm(props: &AddExpenseFormProps) -> Html {
    let AddExpenseFormProps {
        categories,
        on_submit,
        on_cancel,
    } = props;

    let state = use_reducer(|| FormState::default());

    let on_change_amount = {
        let state = state.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_f64(&e);
            tracing::info!("on_change_amount: {}", value);
            state.dispatch(FormAction::EditAmount(value));
        })
    };

    let on_change_description = {
        let state = state.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_string(&e);
            tracing::info!("on_change_description: {}", value);
            state.dispatch(FormAction::EditDescription(value));
        })
    };

    let on_change_category = {
        let state = state.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_usize(&e);
            tracing::info!("on_change_category: {}", value);
            state.dispatch(FormAction::EditCategory(value));
        })
    };

    let on_change_notes = {
        let state = state.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_string(&e);
            tracing::info!("on_change_notes: {}", value);
            state.dispatch(FormAction::EditNotes(value));
        })
    };

    let handle_submit = {
        let on_submit = on_submit.clone();
        let amount = state.amount;
        let description = state.description.clone();
        let category = state.category;
        let notes = state.notes.clone();

        Callback::from(move |_| {
            on_submit.emit((amount, description.clone(), category, notes.clone()))
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
                        value={fmt_currency(state.amount)}
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
                    onclick={handle_submit}
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
