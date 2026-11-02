use crate::prelude::*;
use web_sys::{HtmlInputElement, InputEvent};

#[derive(PartialEq, Properties)]
pub struct InputAmountProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub value: f64,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub autofocus: bool,
}

#[function_component]
pub fn InputAmount(props: &InputAmountProps) -> Html {
    let InputAmountProps {
        class,
        value,
        onchange,
        readonly,
        autofocus,
    } = props;

    let input_ref = use_node_ref();
    let autofocus = *autofocus;
    {
        let input_ref = input_ref.clone();

        use_effect_with((), move |_| {
            if autofocus {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                }
            }
        });
    }

    let focused = use_state(|| false);
    let amount = if *focused && !*readonly {
        if *value == 0.0 {
            "".to_string()
        } else {
            value.to_string()
        }
    } else {
        fmt_amount(*value)
    };

    let onfocus = {
        let focused = focused.clone();

        Callback::from(move |_: FocusEvent| {
            tracing::info!("onfocus");
            focused.set(true);
        })
    };

    let onblur = {
        let focused = focused.clone();

        Callback::from(move |_: FocusEvent| {
            tracing::info!("onblur");
            focused.set(false);
        })
    };

    let oninput = {
        let input_ref = input_ref.clone();

        Callback::from(move |_: InputEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let sanitized = sanitize_amount(&input.value());
                if input.value() != sanitized {
                    input.set_value(&sanitized);
                }
            }
        })
    };

    html! {
        <input
            ref={input_ref}
            class={class}
            id="amount"
            name="amount"
            value={amount}
            onchange={onchange}
            onfocus={onfocus}
            onblur={onblur}
            oninput={oninput}
            readonly={*readonly}
        />
    }
}
