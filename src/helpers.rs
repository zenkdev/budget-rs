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
