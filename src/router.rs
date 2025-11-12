use crate::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/add-expense")]
    AddExpense,
    #[at("/view-reports")]
    ViewReports,
    #[at("/manage-limits")]
    ManageLimits,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Dashboard /> },
        Route::AddExpense => html! { <AddExpense /> },
        Route::ViewReports => html! { <ViewReports /> },
        Route::ManageLimits => html! { <ManageLimits /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn Router() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} /> // <- must be child of <HashRouter>
        </HashRouter>
    }
}
