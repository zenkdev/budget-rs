extern crate chrono;
use chrono::{Local, Months};
use gloo::timers::callback::Interval;
use yew::prelude::*;

fn get_current_time() -> String {
    let now = Local::now()
        .checked_add_months(Months::new(1000 * 12))
        .unwrap();
    format!("{}", now.format("%Y.%m.%d // %H:%M:%S"))
}

#[function_component]
pub fn Footer() -> Html {
    let time = use_state(get_current_time);

    {
        let time = time.clone();
        use_effect_with((), |_| {
            Interval::new(1000, move || time.set(get_current_time())).forget();
        });
    }

    html! {
        <footer class="mt-8 border-t border-solid border-primary/30 px-10 py-3 text-sm text-primary/80">
            <div class="flex justify-between items-center">
                <span>{ "STATUS: ONLINE | USER: VAULT_DWELLER_101" }</span>
                <div class="flex items-center gap-2">
                    <span>{ time.as_str() }</span>
                    <span class="animate-pulse">{ "█" }</span>
                </div>
            </div>
        </footer>
    }
}
