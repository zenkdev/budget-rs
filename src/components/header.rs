use crate::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
        <header class="flex flex-col sm:flex-row items-start sm:items-center justify-between whitespace-nowrap border-b border-solid border-primary/30 px-4 sm:px-10 py-3 text-glow">
            <div class="flex items-center gap-4 text-primary mb-2 sm:mb-0">
                <div class="size-6 text-primary">
                    <svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <g>
                            <circle cx="32" cy="32" r="22" stroke="#19e619" stroke-width="8" fill="none"/>
                            <g>
                            <rect x="28" y="0" width="8" height="8" fill="#19e619"/>
                            <rect x="28" y="56" width="8" height="8" fill="#19e619"/>
                            <rect x="56" y="28" width="8" height="8" fill="#19e619"/>
                            <rect x="0" y="28" width="8" height="8" fill="#19e619"/>
                            <rect x="46.5" y="7.5" width="8" height="10" transform="rotate(45 50.5 13.5)" fill="#19e619"/>
                            <rect x="7.5" y="47" width="8" height="10" transform="rotate(45 13.5 52.5)" fill="#19e619"/>
                            <rect x="46.5" y="46.5" width="8" height="10" transform="rotate(-45 50.5 50.5)" fill="#19e619"/>
                            <rect x="12.5" y="8.5" width="8" height="10" transform="rotate(-45 13.5 16.5)" fill="#19e619"/>
                            </g>
                        </g>
                        <text x="32" y="44" font-size="36" font-weight="bold" text-anchor="middle" fill="#19e619" font-family="Arial">{ "$" }</text>
                    </svg>
                </div>
                <h2 class="text-primary text-lg font-bold leading-tight tracking-[-0.015em]">
                    { "V.B.S. (VAULT BUDGETING SYSTEM) v1.0" }
                </h2>
            </div>
            <div class="flex flex-1 sm:justify-end gap-4 sm:gap-8">
                <div class="flex items-center gap-4 sm:gap-9">
                    <Link<Route> classes="text-primary text-glow text-sm font-medium leading-normal hover:text-white" to={Route::DataTransfer}>
                        { "[DATA]" }
                    </Link<Route>>
                    <a class="text-primary text-sm font-medium leading-normal hover:text-white" href="#">{"[LOGOUT]"}</a>
                </div>
            </div>
        </header>
    }
}
