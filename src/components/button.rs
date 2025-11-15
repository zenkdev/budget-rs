use crate::prelude::*;

#[derive(PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Cancel,
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub button_type: String,
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    pub children: Children,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps {
        class,
        variant,
        button_type,
        onclick,
        disabled,
        children,
    } = props;

    html! {
        <button
            class={classes!(
                "flex", "cursor-pointer", "items-center", "justify-center", "overflow-hidden", "rounded", "h-12", "px-6", "text-base", "font-bold", "leading-normal", "tracking-[0.015em]", "transition-colors", "duration-200",
                variant.eq(&ButtonVariant::Primary).then_some("bg-primary/90 hover:bg-primary text-background-dark"),
                variant.eq(&ButtonVariant::Secondary).then_some("bg-[#244724] text-[#93c893] hover:text-primary"),
                variant.eq(&ButtonVariant::Cancel).then_some("border border-[#346534] text-[#93c893] hover:border-primary hover:text-primary"),
                class.clone(),
            )}
            type={button_type.clone()}
            onclick={onclick}
            disabled={*disabled}
        >
            { children.clone() }
        </button>
    }
}
