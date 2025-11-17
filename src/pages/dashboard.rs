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
                        <section>
                            <h3 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4 text-glow">
                                { "// COMMANDS //" }
                            </h3>
                            <div class="p-4 flex flex-col sm:flex-row gap-4">
                                <CommandLink to={Route::AddTransaction}>
                                    { ">_ ADD NEW TRANSACTION" }
                                </CommandLink>
                                <CommandLink to={Route::ViewReports}                                >
                                    { ">_ VIEW REPORTS" }
                                </CommandLink>
                                <CommandLink to={Route::ManageLimits}>
                                    { ">_ MANAGE LIMITS" }
                                </CommandLink>
                            </div>
                        </section>
                    </main>
                    <Footer/>
                </div>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct CommandLinkProps {
    to: Route,
    children: Children,
}

#[function_component]
fn CommandLink(props: &CommandLinkProps) -> Html {
    let CommandLinkProps { to, children } = props;

    html! {
        <Link<Route>
            classes="flex-1 text-left p-4 border border-primary/30 rounded hover:bg-primary/20 hover:text-white transition-colors duration-200"
            to={to.clone()}
        >
            <span class="font-bold">
                { children.clone() }
            </span>
        </Link<Route>>
    }
}
