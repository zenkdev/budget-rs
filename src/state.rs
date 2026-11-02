use chrono::{DateTime, Local};
use gloo::file::{Blob, ObjectUrl};
use gloo::storage::{LocalStorage, Storage};
use gloo::utils::document;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::rc::Rc;
use web_sys::{wasm_bindgen::JsCast, File, FileReader, HtmlAnchorElement};
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    #[serde(default)]
    pub id: usize,
    pub amount: f64,
    pub date: DateTime<Local>,
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
        let now = Local::now();
        Self {
            transactions: vec![
                Transaction {
                    id: 1,
                    amount: 45.50,
                    date: now,
                    description: "Super-Duper Mart Rations".to_string(),
                    category: 1,
                    notes: "".to_string(),
                },
                Transaction {
                    id: 2,
                    amount: 120.00,
                    date: now,
                    description: "Hydroponic Vegetable Seeds".to_string(),
                    category: 2,
                    notes: "".to_string(),
                },
                Transaction {
                    id: 3,
                    amount: 75.00,
                    date: now,
                    description: "Gas for Power Generator".to_string(),
                    category: 2,
                    notes: "".to_string(),
                },
                Transaction {
                    id: 4,
                    amount: 30.00,
                    date: now,
                    description: "Red Rocket Fuel".to_string(),
                    category: 3,
                    notes: "".to_string(),
                },
                Transaction {
                    id: 5,
                    amount: 80.00,
                    date: now,
                    description: "Repair of Shelter Door Lock".to_string(),
                    category: 5,
                    notes: "".to_string(),
                },
                Transaction {
                    id: 6,
                    amount: 200.00,
                    date: now,
                    description: "Purchase of 10mm Rounds".to_string(),
                    category: 6,
                    notes: "".to_string(),
                },
                Transaction {
                    id: 7,
                    amount: 60.00,
                    date: now,
                    description: "Medicine and Stimpaks".to_string(),
                    category: 4,
                    notes: "".to_string(),
                },
                Transaction {
                    id: 8,
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

fn next_id(transactions: &[Transaction]) -> usize {
    transactions.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

pub fn migrate_transaction_ids(state: &mut State) {
    let mut next = next_id(&state.transactions);
    for transaction in &mut state.transactions {
        if transaction.id == 0 {
            transaction.id = next;
            next += 1;
        }
    }
}

pub fn latest_category_for_description(transactions: &[Transaction], description: &str) -> Option<usize> {
    let target = description.trim().to_lowercase();
    if target.is_empty() {
        return None;
    }

    transactions
        .iter()
        .filter(|t| t.description.trim().to_lowercase() == target)
        .max_by_key(|t| t.date)
        .map(|t| t.category)
}

pub enum Action {
    AddTransaction(Transaction),
    EditTransaction(usize, Transaction),
    EditLimits((f64, Vec<Category>)),
    MoveCategoryTransactions((usize, usize)),
    Load(State),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::AddTransaction(mut transaction) => {
                let mut transactions = self.transactions.clone();
                transaction.id = next_id(&transactions);
                transactions.push(transaction);

                State {
                    transactions,
                    categories: self.categories.clone(),
                    monthly_limit: self.monthly_limit,
                }
                .into()
            }
            Action::EditTransaction(id, mut transaction) => {
                let mut transactions = self.transactions.clone();
                match transactions.iter().position(|t| t.id == id) {
                    Some(pos) => {
                        transaction.id = id;
                        transactions[pos] = transaction;
                    }
                    None => {
                        tracing::error!("EditTransaction: transaction with id {} not found", id);
                    }
                }

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
            Action::MoveCategoryTransactions((from, to)) => {
                if from == to
                    || !self.categories.iter().any(|category| category.id == from)
                    || !self.categories.iter().any(|category| category.id == to)
                {
                    tracing::error!(
                        "MoveCategoryTransactions: invalid move from {} to {}",
                        from,
                        to
                    );
                    return self;
                }

                let mut transactions = self.transactions.clone();
                for transaction in &mut transactions {
                    if transaction.category == from {
                        transaction.category = to;
                    }
                }

                State {
                    transactions,
                    categories: self.categories.clone(),
                    monthly_limit: self.monthly_limit,
                }
                .into()
            }
            Action::Load(mut state) => {
                migrate_transaction_ids(&mut state);
                state.into()
            }
        }
    }
}

const KEY: &str = "@budget-rs/app-state";

#[hook]
pub fn use_app_state() -> UseReducerHandle<State> {
    let state = use_reducer(|| {
        let mut state = LocalStorage::get(KEY).unwrap_or_else(|_| State::default());
        migrate_transaction_ids(&mut state);
        state
    });

    tracing::info!("Transactions loaded: {}", state.transactions.len());

    use_effect_with(state.clone(), |state| {
        tracing::info!("Saving transactions: {}", state.transactions.len());
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

pub type DispatchState = Callback<Action>;

pub fn parse_file(file: File, on_success: Callback<State>) {
    let format = if file.type_() == "text/csv" {
        FileFormat::Csv
    } else if file.type_() == "application/json" {
        FileFormat::Json
    } else {
        panic!("Unsupported file type: {}", file.type_());
    };
    let file_reader = FileReader::new().unwrap();
    let value = file_reader.clone();
    file_reader.set_onloadend(Some(
        &wasm_bindgen::closure::Closure::once_into_js(move |_: Event| {
            let result = value.result().unwrap();
            let content = result.as_string().unwrap(); // Assuming text file
            let state: State = match format {
                FileFormat::Csv => crate::csv::csv_to_state(content),
                FileFormat::Json => serde_json::from_str(&content).unwrap(),
            };
            on_success.emit(state);
        })
        .unchecked_into(),
    ));
    file_reader.read_as_text(&file).unwrap();
}

#[derive(PartialEq, Clone)]
pub enum FileFormat {
    Csv,
    Json,
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileFormat::Csv => write!(f, "csv"),
            FileFormat::Json => write!(f, "json"),
        }
    }
}

pub fn save_to_file(state: State, format: FileFormat) {
    let content = match format {
        FileFormat::Csv => crate::csv::state_to_csv(state.clone()),
        FileFormat::Json => serde_json::to_string(&state).expect("Failed to convert to JSON"),
    };
    let blob = Blob::new(content.as_str());
    let url = ObjectUrl::from(blob);
    let a = document()
        .create_element("a")
        .expect("Failed to create anchor element")
        .dyn_into::<HtmlAnchorElement>()
        .expect("Failed to cast element to anchor element");
    a.set_href(&url);
    a.set_download(
        format!(
            "budget_data_{}.{}",
            Local::now().format("%Y%m%d%H%M%S"),
            format
        )
        .as_str(),
    );
    a.click();
}
