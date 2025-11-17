use crate::state::Transaction;
use chrono::{DateTime, Datelike, Local, Months, NaiveDate};
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

pub fn target_input_value_date(e: &Event) -> DateTime<Local> {
    let input: HtmlInputElement = e.target_unchecked_into();
    let date = NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d");
    DateTime::<Local>::from_naive_utc_and_offset(date.unwrap().into(), *Local::now().offset())
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

pub fn get_start_of_month() -> DateTime<Local> {
    let now = Local::now();
    DateTime::<Local>::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .unwrap()
            .into(),
        *now.offset(),
    )
}

pub fn get_category_spent_this_month(cat_id: usize, transactions: &[Transaction]) -> f64 {
    let start_of_month = get_start_of_month();

    transactions
        .iter()
        .filter(|t| t.category == cat_id && t.date >= start_of_month)
        .fold(0.0, |acc, t| acc + t.amount)
}

pub fn get_percent(amount: f64, total: f64) -> i32 {
    if total > 0.0 {
        (amount / total * 100.0) as i32
    } else {
        0
    }
}
