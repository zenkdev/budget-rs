use crate::state::Transaction;
use chrono::{DateTime, Local, Months};
use numfmt::{Formatter, Precision};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub fn fmt_amount(amount: f64) -> String {
    let f = Formatter::new() // start with blank representation
        .separator(',')
        .unwrap()
        .prefix("$")
        .unwrap()
        .precision(Precision::Decimals(2));
    f.fmt_string(amount)
}

pub fn fmt_date(date: DateTime<Local>) -> String {
    let fixed = date.checked_add_months(Months::new(1000 * 12)).unwrap();

    format!("{}", fixed.format("%Y.%m.%d"))
}

pub fn target_input_value_string(e: &Event) -> String {
    let input: HtmlInputElement = e.target_unchecked_into();
    input.value()
}

pub fn target_input_value_amount(e: &Event) -> f64 {
    let input: HtmlInputElement = e.target_unchecked_into();
    input
        .value()
        .replace("$", "")
        .replace(",", "")
        .parse::<f64>()
        .unwrap_or(0.0)
}

pub fn target_input_value_usize(e: &Event) -> usize {
    let input: HtmlInputElement = e.target_unchecked_into();
    input.value().parse::<usize>().unwrap_or(0)
}

pub fn pad_right(s: &str, width: usize, pad_char: char) -> String {
    let current_len = s.chars().count();
    if current_len >= width {
        s.to_string()
    } else {
        let padding_needed = width - current_len;
        let padding_string: String = std::iter::repeat_n(pad_char, padding_needed).collect();
        format!("{}{}", s, padding_string)
    }
}

pub fn get_category_spent(cat_id: usize, transactions: &[Transaction]) -> f64 {
    transactions
        .iter()
        .filter(|t| t.category == cat_id)
        .fold(0.0, |acc, t| acc + t.amount)
}
