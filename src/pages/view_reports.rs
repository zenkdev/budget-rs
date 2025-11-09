use crate::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ViewReportsProps {
    pub open: bool,
    pub on_close: Callback<MouseEvent>,
}

#[function_component]
pub fn ViewReports(props: &ViewReportsProps) -> Html {
    let ViewReportsProps { open, on_close } = props;

    html! {
    <Dialog open={open}>
        <div class="layout-container flex h-full grow flex-col">
            <div class="px-4 sm:px-8 md:px-16 lg:px-24 xl:px-40 flex flex-1 justify-center py-5">
                <div class="layout-content-container flex flex-col max-w-[960px] flex-1">
                    // <!-- Back Button -->
                    <div class="flex px-4 py-3 justify-start">
                        <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden h-10 px-4 bg-transparent text-[#93c893] hover:text-primary text-sm font-bold leading-normal tracking-[0.015em]" onclick={on_close}>
                            <span class="truncate">{"[ < BACK ]"}</span>
                        </button>
                    </div>
                    // <!-- Page Heading -->
                    <div class="flex flex-wrap justify-between gap-3 p-4">
                        <div class="flex min-w-72 flex-col gap-3">
                            <p class="text-primary text-4xl font-black leading-tight tracking-[-0.033em]">{"[ FINANCIAL OVERVIEW ]"}</p>
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
                    // <!-- Spacer -->
                    <div class="h-8"></div>
                    // <!-- Financial Overview -->
                    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                        <div class="lg:col-span-2 flex flex-col gap-6">
                            <div class="border border-[#244724] p-4 rounded">
                                <div class="flex flex-col gap-3">
                                    <div class="flex gap-6 justify-between items-center">
                                        <p class="text-white text-base font-medium leading-normal">{"Monthly Rationing Allowance"}</p>
                                        <p class="text-white text-sm font-normal leading-normal">{"65%"}</p>
                                    </div>
                                    <div class="rounded bg-[#346534]">
                                        <div class="h-2 rounded bg-primary" style="width: 65%;"></div>
                                    </div>
                                    <p class="text-[#93c893] text-sm font-normal leading-normal">{"$1300 / $2000 Spent"}</p>
                                </div>
                            </div>
                            <div class="border border-[#244724] p-4 rounded flex flex-wrap gap-4">
                                <div class="flex min-w-72 flex-1 flex-col gap-2">
                                    <p class="text-white text-base font-medium leading-normal">{"Spending Breakdown"}</p>
                                    <p class="text-primary tracking-light text-[32px] font-bold leading-tight truncate">{"$1300.00"}</p>
                                    <div class="flex gap-1">
                                        <p class="text-[#93c893] text-base font-normal leading-normal">{"This Month"}</p>
                                        <p class="text-primary text-base font-medium leading-normal">{"+5%"}</p>
                                    </div>
                                    <div class="grid min-h-[180px] grid-flow-col gap-6 grid-rows-[1fr_auto] items-end justify-items-center px-3">
                                        <div class="border-primary bg-[#244724] border-t-2 w-full" style="height: 25%;"></div>
                                        <p class="text-[#93c893] text-[13px] font-bold leading-normal tracking-[0.015em]">{"Food"}</p>
                                        <div class="border-primary bg-[#244724] border-t-2 w-full" style="height: 40%;"></div>
                                        <p class="text-[#93c893] text-[13px] font-bold leading-normal tracking-[0.015em]">{"Utilities"}</p>
                                        <div class="border-primary bg-[#244724] border-t-2 w-full" style="height: 15%;"></div>
                                        <p class="text-[#93c893] text-[13px] font-bold leading-normal tracking-[0.015em]">{"Transport"}</p>
                                        <div class="border-primary bg-[#244724] border-t-2 w-full" style="height: 10%;"></div>
                                        <p class="text-[#93c893] text-[13px] font-bold leading-normal tracking-[0.015em]">{"Misc"}</p>
                                        <div class="border-primary bg-[#244724] border-t-2 w-full" style="height: 10%;"></div>
                                        <p class="text-[#93c893] text-[13px] font-bold leading-normal tracking-[0.015em]">{"Meds"}</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="lg:col-span-1 flex flex-col gap-6">
                            <div class="border border-[#244724] p-4 rounded">
                                <h3 class="text-white text-lg font-bold mb-4">{"// SUMMARY //"}</h3>
                                <div class="flex flex-col gap-3 text-sm">
                                    <div class="flex justify-between">
                                        <span class="text-[#93c893]">{"[TOTAL INCOME]:"}</span>
                                        <span class="text-white font-bold">{"$2000.00"}</span>
                                    </div>
                                    <div class="flex justify-between">
                                        <span class="text-[#93c893]">{"[TOTAL SPENT]:"}</span>
                                        <span class="text-white font-bold">{"$1300.00"}</span>
                                    </div>
                                    <div class="flex justify-between">
                                        <span class="text-[#93c893]">{"[NET SAVINGS]:"}</span>
                                        <span class="text-primary font-bold">{"$700.00"}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="border border-[#244724] p-4 rounded">
                                <h3 class="text-white text-lg font-bold mb-4">{"// RECENT TRANSACTIONS //"}</h3>
                                <div class="overflow-x-auto">
                                    <table class="w-full text-left text-sm">
                                        <thead>
                                            <tr class="text-[#93c893] border-b border-[#346534]">
                                                <th class="py-2 px-1">{"[DATE]"}</th>
                                                <th class="py-2 px-1">{"[DESC]"}</th>
                                                <th class="py-2 px-1 text-right">{"[AMT]"}</th>
                                            </tr>
                                        </thead>
                                        <tbody class="text-white divide-y divide-[#244724]">
                                            <tr>
                                                <td class="py-2 px-1">{"10-23-77"}</td>
                                                <td class="py-2 px-1">{"Super-Duper Mart"}</td>
                                                <td class="py-2 px-1 text-right text-red-400">{"-$58.12"}</td>
                                            </tr>
                                            <tr>
                                                <td class="py-2 px-1">{"10-22-77"}</td>
                                                <td class="py-2 px-1">{"Red Rocket Fuel"}</td>
                                                <td class="py-2 px-1 text-right text-red-400">{"-$35.50"}</td>
                                            </tr>
                                            <tr>
                                                <td class="py-2 px-1">{"10-22-77"}</td>
                                                <td class="py-2 px-1">{"Water Bill"}</td>
                                                <td class="py-2 px-1 text-right text-red-400">{"-$120.00"}</td>
                                            </tr>
                                            <tr>
                                                <td class="py-2 px-1">{"10-21-77"}</td>
                                                <td class="py-2 px-1">{"Stimpak Purchase"}</td>
                                                <td class="py-2 px-1 text-right text-red-400">{"-$75.00"}</td>
                                            </tr>
                                            <tr>
                                                <td class="py-2 px-1">{"10-20-77"}</td>
                                                <td class="py-2 px-1">{"Nuka-Cola"}</td>
                                                <td class="py-2 px-1 text-right text-red-400">{"-$10.00"}</td>
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>
                                <button class="mt-4 flex w-full cursor-pointer items-center justify-center overflow-hidden rounded h-10 px-4 bg-[#244724] hover:bg-primary/30 text-white text-sm font-bold leading-normal tracking-[0.015em]">
                                    <span class="truncate">{"[ ADD NEW TRANSACTION ]"}</span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </Dialog>
    }
}
