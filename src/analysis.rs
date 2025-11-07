use yew::prelude::*;

#[function_component]
pub fn Analysis() -> Html {
    html! {
    <section class="flex flex-col">
        <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">{"// CATEGORY ANALYSIS //"}</h3>
        <div class="p-4 border border-primary/30 rounded flex-1">
            <div class="flex justify-between gap-x-6 py-2">
                <p class="text-primary/90 text-sm font-normal leading-normal">{"> Food & Rations.........."}</p>
                <p class="text-primary text-sm font-normal leading-normal text-right">{"$550.25"}</p>
            </div>
            <div class="flex justify-between gap-x-6 py-2">
                <p class="text-primary/90 text-sm font-normal leading-normal">{"> Utilities..............."}</p>
                <p class="text-primary text-sm font-normal leading-normal text-right">{"$210.50"}</p>
            </div>
            <div class="flex justify-between gap-x-6 py-2">
                <p class="text-primary/90 text-sm font-normal leading-normal">{"> Travel.................."}</p>
                <p class="text-primary text-sm font-normal leading-normal text-right">{"$120.00"}</p>
            </div>
            <div class="flex justify-between gap-x-6 py-2">
                <p class="text-primary/90 text-sm font-normal leading-normal">{"> Misc. Supplies.........."}</p>
                <p class="text-primary text-sm font-normal leading-normal text-right">{"$315.00"}</p>
            </div>
            <div class="flex justify-between gap-x-6 py-2">
                <p class="text-primary/90 text-sm font-normal leading-normal">{"> Shelter Maintenance....."}</p>
                <p class="text-primary text-sm font-normal leading-normal text-right">{"$850.00"}</p>
            </div>
            <div class="flex justify-between gap-x-6 py-2">
                <p class="text-primary/90 text-sm font-normal leading-normal">{"> Ammunition & Defense...."}</p>
                <p class="text-primary text-sm font-normal leading-normal text-right">{"$105.00"}</p>
            </div>
        </div>
    </section>
    }
}
