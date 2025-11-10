use crate::prelude::*;

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <div class="layout-container flex h-full grow flex-col">
            <div class="px-4 sm:px-10 md:px-20 lg:px-40 flex flex-1 justify-center py-5">
                <div class="layout-content-container flex flex-col max-w-[960px] flex-1">
                    <Header/>
                    <main class="flex flex-col gap-4 mt-4">
                        <MonthlyOverview />
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                            <CategoryAnalysis />
                            <TransactionLogs />
                        </div>
                        <Commands />
                    </main>
                    <Footer/>
                </div>
            </div>
        </div>
    }
}
