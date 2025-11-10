use crate::prelude::*;

#[function_component]
pub fn App() -> Html {
    let state = use_app_state();

    let dispatch = {
        let state = state.clone();
        Callback::from(move |action: Action| state.dispatch(action))
    };

    html! {
        <div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden">
            <div class="scanlines"></div>
            <ContextProvider<State> context={(*state).clone()}>
                <ContextProvider<DispatchState> context={dispatch}>
                    <Router />
                </ContextProvider<DispatchState>>
            </ContextProvider<State>>
        </div>
    }
}
