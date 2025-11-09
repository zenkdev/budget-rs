use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ManageLimitsProps {
    pub open: bool,
    pub on_close: Callback<MouseEvent>,
    pub categories: Vec<Category>,
    pub monthly_limit: f64,
    pub on_submit: Callback<(f64, Vec<Category>)>,
}

#[function_component]
pub fn ManageLimits(props: &ManageLimitsProps) -> Html {
    let ManageLimitsProps {
        open,
        on_close,
        categories,
        monthly_limit,
        on_submit,
    } = props;

    html! {
        <Dialog open={open}>
            <div class="layout-container flex h-full grow flex-col">
                <div class="px-4 sm:px-10 md:px-20 lg:px-40 flex flex-1 justify-center py-5">
                    <div class="layout-content-container flex flex-col max-w-[960px] flex-1">
                        // <!-- Back Button -->
                        <div class="flex px-4 py-3 justify-start">
                            <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden h-10 px-4 bg-transparent text-[#93c893] hover:text-primary text-sm font-bold leading-normal tracking-[0.015em]" onclick={on_close}>
                                <span class="truncate">{"[ < BACK ]"}</span>
                            </button>
                        </div>
                        // <!-- Page Heading -->
                        <div class="flex flex-wrap justify-between gap-3 p-4">
                            <div class="flex min-w-72 flex-col gap-3">
                                <p class="text-primary text-4xl font-black leading-tight tracking-[-0.033em]">{"[ BUDGET LIMIT CONFIGURATION ]"}</p>
                                <p class="text-[#93c893] text-base font-normal leading-normal">{"> SET SPENDING THRESHOLDS. SYSTEM WILL ALERT WHEN LIMITS ARE APPROACHED."}</p>
                            </div>
                        </div>
                        <ManageLimitsForm
                            categories={categories.clone()}
                            monthly_limit={monthly_limit.clone()}
                            on_submit={on_submit.clone()}
                        />
                        // <!-- Status Line -->
                        <div class="px-4 py-6">
                            <p class="text-primary text-base font-normal leading-normal">{"STATUS: AWAITING USER INPUT"}<span class="blinking-cursor">{"_"}</span></p>
                        </div>
                    </div>
                </div>
            </div>
        </Dialog>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct FormState {
    categories: Vec<Category>,
    monthly_limit: f64,
}

impl Eq for FormState {}

enum FormAction {
    EditCategory((usize, String, f64)),
    EditMonthlyLimit(f64),
}

impl Reducible for FormState {
    type Action = FormAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            FormAction::EditCategory((id, name, limit)) => {
                let mut categories = self.categories.clone();

                let category = categories.iter_mut().find(|category| category.id == id);
                if let Some(category) = category {
                    category.name = name;
                    category.limit = limit;
                }
                FormState {
                    categories,
                    monthly_limit: self.monthly_limit,
                }
                .into()
            }
            FormAction::EditMonthlyLimit(monthly_limit) => FormState {
                categories: self.categories.clone(),
                monthly_limit,
            }
            .into(),
        }
    }
}

#[derive(PartialEq, Properties, Clone)]
struct ManageLimitsFormProps {
    pub categories: Vec<Category>,
    pub monthly_limit: f64,
    pub on_submit: Callback<(f64, Vec<Category>)>,
}

#[function_component]
fn ManageLimitsForm(props: &ManageLimitsFormProps) -> Html {
    let ManageLimitsFormProps {
        categories,
        monthly_limit,
        on_submit,
    } = props;

    let state = use_reducer(|| FormState {
        categories: categories.clone(),
        monthly_limit: monthly_limit.clone(),
    });

    let on_change_monthly_limit = {
        let state = state.clone();

        Callback::from(move |e: Event| {
            let value = target_input_value_amount(&e);
            tracing::info!("on_change_monthly_limit: {}", value);
            state.dispatch(FormAction::EditMonthlyLimit(value));
        })
    };

    let on_edit_category = {
        let state = state.clone();

        Callback::from(move |value: (usize, String, f64)| {
            tracing::info!("on_edit_category: {:?}", value);
            state.dispatch(FormAction::EditCategory(value));
        })
    };

    let handle_submit = {
        let on_submit = on_submit.clone();
        let monthly_limit = state.monthly_limit.clone();
        let categories = state.categories.clone();

        Callback::from(move |_| on_submit.emit((monthly_limit, categories.clone())))
    };

    let handle_reset = {
        let on_submit = on_submit.clone();
        let state = State::default();

        Callback::from(move |_| on_submit.emit((state.monthly_limit, state.categories.clone())))
    };

    html! {
        <form class="flex flex-col gap-8">
            // <!-- Overall Limit Input -->
            <div class="flex max-w-[480px] flex-wrap items-end gap-4 px-4 py-3">
                <label class="flex flex-col min-w-40 flex-1">
                    <p class="text-primary text-base font-medium leading-normal pb-2">{"> OVERALL MONTHLY LIMIT:"}</p>
                    <div class="relative">
                        <input
                            class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden text-primary focus:outline-0 focus:ring-0 border border-[#346534] bg-[#1a321a] focus:border-primary h-14 placeholder:text-[#93c893] p-[15px] text-base font-normal leading-normal pr-4"
                            value={fmt_amount(state.monthly_limit)}
                            onchange={on_change_monthly_limit}
                        />
                        <span class="blinking-cursor absolute right-3 top-1/2 -translate-y-1/2 text-primary font-bold">{"_"}</span>
                    </div>
                </label>
            </div>
            // <!-- Category Limits Table -->
            <div class="px-4 py-3 @container">
                <div class="flex overflow-hidden border border-[#346534] bg-background-dark">
                    <table class="flex-1 w-full">
                        <thead>
                            <tr class="bg-[#1a321a]">
                                <th class="px-4 py-3 text-left text-primary w-[30%] sm:w-[25%] text-sm font-medium leading-normal">{"CATEGORY"}</th>
                                <th class="px-4 py-3 text-left text-primary w-[30%] sm:w-[25%] text-sm font-medium leading-normal">{"LIMIT"}</th>
                                <th class="px-4 py-3 text-left text-primary w-[40%] sm:w-[50%] text-sm font-medium leading-normal">{"CURRENT SPEND"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {for state.categories.iter().cloned().map(|category| html! {
                                <CategoryEdit category={category} on_edit={on_edit_category.clone()} />
                            })}
                        </tbody>
                    </table>
                </div>
            </div>
            // <!-- Action Buttons -->
            <div class="flex justify-stretch">
                <div class="flex flex-1 gap-3 flex-wrap px-4 py-3 justify-start">
                    <button
                        class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden h-10 px-4 bg-primary text-background-dark hover:bg-opacity-80 text-sm font-bold leading-normal tracking-[0.015em]"
                        onclick={handle_submit}>
                        <span class="truncate">{"[ SAVE CHANGES ]"}</span>
                    </button>
                    <button
                        class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden h-10 px-4 bg-[#244724] text-[#93c893] hover:text-primary text-sm font-bold leading-normal tracking-[0.015em]"
                        onclick={handle_reset}>
                        <span class="truncate">{"[ RESET TO DEFAULTS ]"}</span>
                    </button>
                </div>
            </div>
        </form>
    }
}

#[derive(PartialEq, Properties, Clone)]
struct CategoryEditProps {
    pub category: Category,
    pub on_edit: Callback<(usize, String, f64)>,
}

#[function_component]
fn CategoryEdit(props: &CategoryEditProps) -> Html {
    let CategoryEditProps { category, on_edit } = props;

    let id = category.id;

    let on_change = {
        let edit = on_edit.clone();
        let name = category.name.clone();

        move |e: Event| {
            let value = target_input_value_amount(&e);
            edit.emit((id, name.clone(), value));
        }
    };

    html! {
    <tr class="border-t border-t-[#346534]">
        <td class="h-[72px] px-4 py-2 text-[#93c893] text-sm font-normal leading-normal">{category.name.clone()}</td>
        <td class="h-[72px] px-4 py-2">
            <input
                class="form-input w-full min-w-0 resize-none overflow-hidden text-primary focus:outline-0 focus:ring-0 border border-[#346534] bg-[#1a321a] focus:border-primary h-12 placeholder:text-[#93c893] p-3 text-sm font-normal leading-normal"
                value={fmt_amount(category.limit)}
                onchange={on_change}
            />
        </td>
        <td class="h-[72px] px-4 py-2 text-sm font-normal leading-normal">
            <p class="text-primary text-sm font-medium leading-normal">{"[█████████░] 90%"}</p>
        </td>
    </tr>
    }
}
