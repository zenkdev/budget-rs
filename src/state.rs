use chrono::{DateTime, Utc};
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
        Self {
            transactions: vec![],
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

pub const KEY: &str = "@budget-rs/app-state";

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

// fn use_app_state() {
//     let state = use_reducer(|| State {
//         entries: LocalStorage::get(KEY).unwrap_or_else(|_| vec![]),
//     });

//     use_effect_with(state.clone(), |state| {
//         LocalStorage::set(KEY, &state.clone()).expect("failed to set");
//     });
// }
