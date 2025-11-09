use chrono::{DateTime, Utc};
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub amount: f64,
    pub date: DateTime<Utc>,
    pub description: String,
    pub category: usize,
    pub notes: String,
}

impl Eq for Transaction {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: usize,
    pub name: String,
    pub limit: f64,
}

impl Eq for Category {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub transactions: Vec<Transaction>,
    pub categories: Vec<Category>,
    pub monthly_limit: f64,
}

impl Eq for State {}

impl Default for State {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            transactions: vec![
                Transaction {
                    amount: 45.50,
                    date: now,
                    description: "Super-Duper Mart Rations".to_string(),
                    category: 1,
                    notes: "".to_string(),
                },
                Transaction {
                    amount: 120.00,
                    date: now,
                    description: "Hydroponic Vegetable Seeds".to_string(),
                    category: 2,
                    notes: "".to_string(),
                },
                Transaction {
                    amount: 75.00,
                    date: now,
                    description: "Gas for Power Generator".to_string(),
                    category: 2,
                    notes: "".to_string(),
                },
                Transaction {
                    amount: 30.00,
                    date: now,
                    description: "Red Rocket Fuel".to_string(),
                    category: 3,
                    notes: "".to_string(),
                },
                Transaction {
                    amount: 80.00,
                    date: now,
                    description: "Repair of Shelter Door Lock".to_string(),
                    category: 5,
                    notes: "".to_string(),
                },
                Transaction {
                    amount: 200.00,
                    date: now,
                    description: "Purchase of 10mm Rounds".to_string(),
                    category: 6,
                    notes: "".to_string(),
                },
                Transaction {
                    amount: 60.00,
                    date: now,
                    description: "Medicine and Stimpaks".to_string(),
                    category: 4,
                    notes: "".to_string(),
                },
                Transaction {
                    amount: 150.00,
                    date: now,
                    description: "Water Chip Replacement".to_string(),
                    category: 2,
                    notes: "".to_string(),
                },
            ],
            categories: vec![
                Category {
                    id: 1,
                    name: "Food & Rations".to_string(),
                    limit: 500.0,
                },
                Category {
                    id: 2,
                    name: "Utilities".to_string(),
                    limit: 500.0,
                },
                Category {
                    id: 3,
                    name: "Travel".to_string(),
                    limit: 500.0,
                },
                Category {
                    id: 4,
                    name: "Misc. Supplies".to_string(),
                    limit: 500.0,
                },
                Category {
                    id: 5,
                    name: "Shelter Maintenance".to_string(),
                    limit: 500.0,
                },
                Category {
                    id: 6,
                    name: "Ammunition & Defense".to_string(),
                    limit: 500.0,
                },
            ],
            monthly_limit: 2000.0,
        }
    }
}

pub enum Action {
    AddTransaction(Transaction),
    EditLimits((f64, Vec<Category>)),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::AddTransaction(transaction) => {
                let mut transactions = self.transactions.clone();
                transactions.push(transaction);

                State {
                    transactions,
                    categories: self.categories.clone(),
                    monthly_limit: self.monthly_limit,
                }
                .into()
            }
            Action::EditLimits((monthly_limit, categories)) => State {
                transactions: self.transactions.clone(),
                monthly_limit,
                categories,
            }
            .into(),
        }
    }
}

const KEY: &str = "@budget-rs/app-state";

#[hook]
pub fn use_app_state() -> UseReducerHandle<State> {
    let state = use_reducer(|| LocalStorage::get(KEY).unwrap_or_else(|_| State::default()));

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

    state
}
