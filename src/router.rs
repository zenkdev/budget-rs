use crate::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/add")]
    AddTransaction,
    #[at("/reports")]
    ViewReports,
    #[at("/limits")]
    ManageLimits,
    #[at("/data")]
    DataTransfer,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Dashboard /> },
        Route::AddTransaction => html! { <AddTransaction /> },
        Route::ViewReports => html! { <ViewReports /> },
        Route::ManageLimits => html! { <ManageLimits /> },
        Route::DataTransfer => html! { <DataTransfer /> },
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
