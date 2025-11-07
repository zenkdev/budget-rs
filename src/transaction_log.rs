use yew::prelude::*;

#[function_component]
pub fn TransactionLog() -> Html {
    html! {
    <section class="flex flex-col">
        <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">{"// TRANSACTION LOGS //"}</h3>
        <div class="p-4 border border-primary/30 rounded flex-1 overflow-x-auto">
            <table class="w-full text-left text-sm">
                <thead>
                    <tr>
                        <th class="py-2 pr-4 font-medium text-primary/80">{"DATE"}</th>
                        <th class="py-2 pr-4 font-medium text-primary/80">{"DESCRIPTION"}</th>
                        <th class="py-2 text-right font-medium text-primary/80">{"AMOUNT"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td class="py-1.5 pr-4 whitespace-nowrap">{"2077.10.23"}</td>
                        <td class="py-1.5 pr-4">{"Super-Duper Mart Rations"}</td>
                        <td class="py-1.5 text-right">{"-$45.50"}</td>
                    </tr>
                    <tr>
                        <td class="py-1.5 pr-4 whitespace-nowrap">{"2077.10.22"}</td>
                        <td class="py-1.5 pr-4">{"Red Rocket Fuel"}</td>
                        <td class="py-1.5 text-right">{"-$30.00"}</td>
                    </tr>
                    <tr>
                        <td class="py-1.5 pr-4 whitespace-nowrap">{"2077.10.22"}</td>
                        <td class="py-1.5 pr-4">{"Water Chip Payment"}</td>
                        <td class="py-1.5 text-right">{"-$150.00"}</td>
                    </tr>
                    <tr>
                        <td class="py-1.5 pr-4 whitespace-nowrap">{"2077.10.21"}</td>
                        <td class="py-1.5 pr-4">{"Moira's Craterside Supply"}</td>
                        <td class="py-1.5 text-right">{"-$78.25"}</td>
                    </tr>
                    <tr>
                        <td class="py-1.5 pr-4 whitespace-nowrap">{"2077.10.20"}</td>
                        <td class="py-1.5 pr-4">{"Brahmin Feed"}</td>
                        <td class="py-1.5 text-right">{"-$12.00"}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </section>
    }
}
