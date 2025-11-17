use crate::prelude::*;
use chrono::{Datelike, Months};
use std::cmp;

#[function_component]
pub fn ViewReports() -> Html {
    let state = use_context::<State>().expect("no ctx found");
    let transactions = state.transactions;
    let monthly_limit = state.monthly_limit;

    let start_of_month = get_start_of_month();
    let expendeture = transactions
        .iter()
        .filter(|t| t.date >= start_of_month)
        .fold(0.0, |acc, t| acc + t.amount);
    let percent = get_percent(expendeture, monthly_limit);

    html! {
        <div class="layout-container flex h-full grow flex-col">
            <div class="px-4 sm:px-8 md:px-16 lg:px-24 xl:px-40 flex flex-1 justify-center py-5">
                <div class="layout-content-container flex flex-col max-w-[960px] flex-1">
                    <div class="flex px-4 py-3 justify-start">
                        <HomeLink variant={HomeLinkVariant::Back} />
                    </div>
                    <div class="flex flex-wrap justify-between gap-3 p-4">
                        <div class="flex min-w-72 flex-col gap-3">
                            <p class="text-primary text-4xl font-black leading-tight tracking-[-0.033em]">
                                { "[ FINANCIAL OVERVIEW ]" }
                            </p>
                        </div>
                    </div>
                    // <div class="flex gap-2">
                    //     <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded h-10 px-4 bg-[#244724] text-white text-sm font-bold leading-normal tracking-[0.015em]">
                    //         <span class="truncate">{"[USER: OVERSEER]"}</span>
                    //     </button>
                    //     <button class="flex max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded h-10 bg-[#244724] text-white gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-2.5">
                    //         <span class="material-symbols-outlined text-xl">{"schedule"}</span>
                    //     </button>
                    //     <button class="flex max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded h-10 bg-[#244724] text-white gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-2.5">
                    //         <span class="material-symbols-outlined text-xl">{"notifications"}</span>
                    //     </button>
                    // </div>
                    <div class="h-8"></div>
                    <div class="flex flex-col gap-6">
                        <div class="border border-[#244724] p-4 rounded">
                            <div class="flex flex-col gap-3">
                                <div class="flex gap-6 justify-between items-center">
                                    <p class="text-white text-base font-medium leading-normal">{"Monthly Rationing Allowance"}</p>
                                    <p class="text-white text-sm font-normal leading-normal">
                                        { format!("{}%", percent) }
                                    </p>
                                </div>
                                <div class="rounded bg-[#346534]">
                                    <div class="h-2 rounded bg-primary" style={format!("width: {}%", cmp::min(percent, 100))}></div>
                                </div>
                                <p class="text-[#93c893] text-sm font-normal leading-normal">
                                    { format!("{} / {} Spent", fmt_amount(expendeture), fmt_amount(monthly_limit)) }
                                </p>
                            </div>
                        </div>
                        <SpendingBreakdown />
                        <MonthBreakdown />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn SpendingBreakdown() -> Html {
    let state = use_context::<State>().expect("no ctx found");
    let transactions = state.transactions;

    let start_of_month = get_start_of_month();
    let expendeture = transactions
        .iter()
        .filter(|t| t.date >= start_of_month)
        .fold(0.0, |acc, t| acc + t.amount);

    let start_of_prev_month = start_of_month.checked_sub_months(Months::new(1)).unwrap();
    let prev_expendeture = transactions
        .iter()
        .filter(|t| t.date >= start_of_prev_month && t.date < start_of_month)
        .fold(0.0, |acc, t| acc + t.amount);
    let prev_percent = get_percent(expendeture - prev_expendeture, prev_expendeture);
    let mut data = state
        .categories
        .iter()
        .map(|category| {
            let spent = get_category_spent_this_month(category.id, &transactions);
            let percent = get_percent(spent, expendeture);
            (category.name.clone(), percent)
        })
        .filter(|(_, percent)| *percent > 0)
        .collect::<Vec<_>>();
    data.sort_by_key(|(_, percent)| *percent);

    let colors = vec![
        "#FFE16B", "#A7DEFF", "#2BFF6E", "#6CC2F0", "#FF4E17", "#5E26FF", "#C4D86C", "#FF9840",
        "#EAFFE0", "#C7B2FF",
    ];
    let mut total_percent = 0;
    let background = data.iter().enumerate().fold(
        "background: conic-gradient(".to_string(),
        |acc, (index, (_, percent))| {
            let color = colors[index % colors.len()];
            total_percent += *percent;

            if index == 0 {
                format!("{}{} {}%,", acc, color, total_percent)
            } else if index < data.len() - 1 {
                format!("{}{} 0 {}%,", acc, color, total_percent)
            } else {
                format!("{}{} 0);", acc, color)
            }
        },
    );

    html! {
        <div class="border border-[#244724] p-4 rounded flex flex-wrap gap-4">
            <div class="flex min-w-72 flex-1 flex-col gap-2">
                <p class="text-white text-base font-medium leading-normal">
                    { "Spending Breakdown" }
                </p>
                <p class="text-primary tracking-light text-[32px] font-bold leading-tight truncate">
                    { fmt_amount(expendeture) }
                </p>
                <div class="flex gap-1">
                    <p class="text-[#93c893] text-base font-normal leading-normal">
                        { "This Month" }
                    </p>
                    <p class="text-primary text-base font-medium leading-normal">
                        { if prev_percent > 0 {
                            format!("+{}%", prev_percent)
                        } else if prev_percent < 0 {
                            format!("{}%", prev_percent)
                        } else {
                            "".to_string()
                        } }
                    </p>
                </div>
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 items-center">
                    <div class="lg:col-span-2 flex flex-col gap-6">
                        <div class="grid min-h-[180px] grid-flow-col gap-6 grid-rows-[1fr_auto] items-end justify-items-center px-3">
                            <div class="w-[300px] h-[300px] m-8 rounded-full" style={background}></div>
                        </div>
                    </div>
                    <ul class="lg:col-span-1 flex flex-col gap-6">
                        { for data.iter().enumerate().map(|(index, (label, percent))| html! {
                            <li class="flex items-center gap-2">
                                <div class="w-4 h-4 rounded-full" style={format!("background: {};", colors[index % colors.len()])}></div>
                                <p class="text-[#93c893] text-[13px] font-bold leading-normal tracking-[0.015em]">
                                    { format!("{} ({}%)", label.clone(), percent) }
                                </p>
                            </li>
                        })}
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn MonthBreakdown() -> Html {
    let state = use_context::<State>().expect("no ctx found");
    let transactions = state.transactions;
    let monthly_limit = state.monthly_limit;

    let start_of_month = get_start_of_month();
    let mut months = vec![start_of_month];
    for i in 1..12 {
        months.push(start_of_month.checked_sub_months(Months::new(i)).unwrap());
    }
    months.reverse();

    let data = months
        .iter()
        .enumerate()
        .map(|(index, month)| {
            let spent = transactions
                .iter()
                .filter(|t| {
                    t.date >= *month && (index >= months.len() - 1 || t.date < months[index + 1])
                })
                .fold(0.0, |acc, t| acc + t.amount);
            (*month, spent)
        })
        .collect::<Vec<_>>();
    let non_zero_data = data
        .iter()
        .filter(|(_, spent)| *spent > 0.0)
        .collect::<Vec<_>>();
    let average = non_zero_data
        .iter()
        .fold(0.0, |acc, (_, spent)| acc + spent)
        / non_zero_data.len() as f64;
    let min = data.iter().fold(
        0.0,
        |acc, (_, spent)| {
            if *spent < acc {
                *spent
            } else {
                acc
            }
        },
    );
    let max = data.iter().fold(
        0.0,
        |acc, (_, spent)| {
            if *spent > acc {
                *spent
            } else {
                acc
            }
        },
    );
    let normalized_data = data
        .iter()
        .map(|(month, spent)| (*month, (spent - min) / (max - min) * 100.0, *spent))
        .collect::<Vec<_>>();

    html! {
        <div class="border border-[#244724] p-4 rounded flex flex-wrap gap-4">
            <div class="flex min-w-72 flex-1 flex-col gap-2">
                <p class="text-white text-base font-medium leading-normal">
                    { "Month Breakdown" }
                </p>
                <p class="text-primary tracking-light text-[32px] font-bold leading-tight truncate">
                    { fmt_amount(average) }
                </p>
                <p class="text-[#93c893] text-base font-normal leading-normal">
                    { "Average Spending" }
                </p>
                <div class="grid min-h-[180px] grid-flow-col gap-6 grid-rows-[1fr_auto] items-end justify-items-center px-3">
                    { for normalized_data.iter().map(|(month, normalized_spent, spent_amount)| html! {
                        <>
                            <div
                                class="w-full flex items-center justify-center tooltip"
                                style={if *spent_amount > monthly_limit {
                                        let ratio = monthly_limit / *spent_amount;
                                        format!(
                                            "min-height: {}%; background: linear-gradient(to top, #244724 {}%, #19e619 {}%);",
                                            normalized_spent,
                                            ratio * 100.0,
                                            ratio * 100.0,
                                        )
                                    } else {
                                        format!( "min-height: {}%; background: #244724;", normalized_spent)
                                    }
                                }
                            >
                                <span class="tooltip-text">
                                    { fmt_amount(*spent_amount) }
                                </span>
                            </div>
                            <p class="text-[#93c893] text-[13px] font-bold leading-normal tracking-[0.015em]">
                                { if month.month() == 1 {
                                    format!("{}", month.year())
                                } else {
                                    month.format("%b").to_string()
                                } }
                            </p>
                        </>
                    })}
                </div>
            </div>
        </div>
    }
}
