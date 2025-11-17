use crate::state::{Category, State, Transaction};
use chrono::{DateTime, Local, NaiveDate};
use csv::{ReaderBuilder, WriterBuilder};
use std::collections::HashMap;

pub fn csv_to_state(content: String) -> State {
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());
    let mut transactions = Vec::new();
    let mut categories: Vec<Category> = Vec::new();
    let mut monthly_limit = 0.0;
    for record in reader.records() {
        let record = record.unwrap();

        tracing::info!("record: {:?}", record);

        let date_str = record[1].to_string().chars().take(10).collect::<String>();
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .or_else(|_| NaiveDate::parse_from_str(&date_str, "%d.%m.%Y"));
        if let Err(err) = date {
            tracing::error!("Error parsing date: {}", err);
            continue;
        }

        let id: usize;
        let name = record[3].to_string();
        let found = categories.iter().find(|c| c.name == name);
        match found {
            Some(c) => {
                id = c.id;
            }
            None => {
                id = categories.len() + 1;
                categories.push(Category {
                    id,
                    name,
                    limit: record[4].parse::<f64>().unwrap_or(0.0),
                });
            }
        }

        transactions.push(Transaction {
            amount: record[0].parse::<f64>().unwrap(),
            date: DateTime::<Local>::from_naive_utc_and_offset(
                date.unwrap().into(),
                *Local::now().offset(),
            ),
            description: record[2].to_string(),
            category: id,
            notes: record[5].to_string(),
        });

        if monthly_limit == 0.0 {
            monthly_limit = record[6].parse::<f64>().unwrap();
        }
    }

    State {
        transactions,
        categories,
        monthly_limit,
    }
}

pub fn state_to_csv(state: State) -> String {
    let transactions = state.transactions.clone();
    let categories = state.categories.clone();
    let monthly_limit = state.monthly_limit;

    let mut writer = WriterBuilder::new().delimiter(b';').from_writer(vec![]);
    writer
        .write_record([
            "Amount",
            "Date",
            "Description",
            "Category",
            "Limit",
            "Notes",
            "Monthly Limit",
        ])
        .expect("Failed to write record");
    let categories_map = categories
        .iter()
        .map(|c| (c.id, (c.name.clone(), c.limit)))
        .collect::<HashMap<usize, (String, f64)>>();
    for transaction in transactions {
        writer
            .write_record(&[
                transaction.amount.to_string(),
                format!("{}", transaction.date.format("%Y-%m-%d")),
                transaction.description,
                categories_map
                    .get(&transaction.category)
                    .expect("Category not found")
                    .0
                    .clone(),
                categories_map
                    .get(&transaction.category)
                    .expect("Category not found")
                    .1
                    .to_string(),
                transaction.notes,
                monthly_limit.to_string(),
            ])
            .expect("Failed to write record");
    }
    writer.flush().unwrap();

    String::from_utf8(writer.into_inner().unwrap()).expect("Failed to convert to string")
}
