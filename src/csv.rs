use crate::prelude::*;
use chrono::{DateTime, Local, NaiveDate};
use csv::{ReaderBuilder, WriterBuilder};
use gloo_file::{Blob, ObjectUrl};
use gloo_utils::document;
use std::collections::HashMap;
use web_sys::{wasm_bindgen::JsCast, File, FileReader, HtmlAnchorElement};

pub fn save_data_as_csv_file(
    transactions: Vec<Transaction>,
    categories: Vec<Category>,
    monthly_limit: f64,
) {
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

    let content =
        String::from_utf8(writer.into_inner().unwrap()).expect("Failed to convert to string");

    let blob = Blob::new(content.as_str());

    let url = ObjectUrl::from(blob);

    let a = document()
        .create_element("a")
        .expect("Failed to create anchor element")
        .dyn_into::<HtmlAnchorElement>()
        .expect("Failed to cast element to anchor element");
    a.set_href(&url);
    a.set_download("budget_data.csv");
    a.click();
}

pub fn parse_csv_file(file: File, on_success: Callback<State>) {
    let file_reader = FileReader::new().unwrap();
    let value = file_reader.clone();
    file_reader.set_onloadend(Some(
        &wasm_bindgen::closure::Closure::once_into_js(move |_: Event| {
            let result = value.result().unwrap();
            let content = result.as_string().unwrap(); // Assuming text file
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
            on_success.emit(State {
                transactions,
                categories,
                monthly_limit,
            });
        })
        .unchecked_into(),
    ));
    file_reader.read_as_text(&file).unwrap();
}
