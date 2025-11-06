use yew::prelude::*;

#[function_component]
pub fn Overview() -> Html {
    html! {
    <section>
        <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">{"// MONTHLY FINANCIAL OVERVIEW //"}</h3>
        <div class="flex flex-col sm:flex-row flex-wrap gap-4 p-4">
            <div class="flex min-w-[158px] flex-1 flex-col gap-2 rounded p-6 border border-primary/30">
                <p class="text-primary/80 text-base font-medium leading-normal">{"Monthly Income"}</p>
                <p class="text-primary tracking-light text-2xl font-bold leading-tight text-glow">{"$4,500.00"}</p>
            </div>
            <div class="flex min-w-[158px] flex-1 flex-col gap-2 rounded p-6 border border-primary/30">
                <p class="text-primary/80 text-base font-medium leading-normal">{"Expenditure"}</p>
                <p class="text-primary tracking-light text-2xl font-bold leading-tight text-glow">{"$2,150.75"}</p>
            </div>
            <div class="flex min-w-[158px] flex-1 flex-col gap-2 rounded p-6 border border-primary/30">
                <p class="text-primary/80 text-base font-medium leading-normal">{"Funds Remaining"}</p>
                <p class="text-primary tracking-light text-2xl font-bold leading-tight text-glow">{"$2,349.25"}</p>
            </div>
        </div>
    </section>
    }
}
