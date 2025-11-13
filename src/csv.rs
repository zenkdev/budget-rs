use crate::prelude::*;
use chrono::{DateTime, Local, NaiveDate};
use csv::{ReaderBuilder, WriterBuilder};
use gloo_file::{Blob, ObjectUrl};
use gloo_utils::document;
use std::collections::HashMap;
use web_sys::{wasm_bindgen::JsCast, File, FileReader, HtmlAnchorElement};

pub fn save_data_as_csv_file(transactions: Vec<Transaction>, categories: Vec<Category>) {
    let mut writer = WriterBuilder::new().delimiter(b';').from_writer(vec![]);
    writer
        .write_record(["Amount", "Date", "Description", "Category", "Notes"])
        .expect("Failed to write record");
    let categories_map = categories
        .iter()
        .map(|c| (c.id, c.name.clone()))
        .collect::<HashMap<usize, String>>();
    for transaction in transactions {
        writer
            .write_record(&[
                transaction.amount.to_string(),
                format!("{}", transaction.date.format("%Y-%m-%d")),
                transaction.description,
                categories_map
                    .get(&transaction.category)
                    .unwrap_or(&"".to_string())
                    .clone(),
                transaction.notes,
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

pub fn parse_csv_file(file: File, on_success: Callback<(Vec<Transaction>, Vec<Category>)>) {
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
            for record in reader.records() {
                let record = record.unwrap();

                tracing::info!("record: {:?}", record);

                let date = NaiveDate::parse_from_str(&record[1], "%Y-%m-%d");
                if let Err(err) = date {
                    tracing::error!("Error parsing date: {}", err);
                    continue;
                }

                let id: usize;
                let category = record[3].to_string();
                let found = categories.iter().find(|c| c.name == category);
                match found {
                    Some(c) => {
                        id = c.id;
                    }
                    None => {
                        id = categories.len() + 1;
                        categories.push(Category {
                            id: id,
                            name: category,
                            limit: 0.0,
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
                    notes: record[4].to_string(),
                });
            }
            on_success.emit((transactions, categories));
        })
        .unchecked_into(),
    ));
    file_reader.read_as_text(&file).unwrap();
}
