use crate::prelude::*;
use csv::WriterBuilder;
use gloo_file::{Blob, ObjectUrl};
use gloo_utils::document;
use std::collections::HashMap;
use web_sys::{wasm_bindgen::JsCast, HtmlAnchorElement};

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
                transaction.date.to_rfc3339(),
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
