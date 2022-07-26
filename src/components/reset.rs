use crate::states::ticker::{TickerState, TickerStateAction};
use yew::prelude::*;

#[function_component]
pub fn Reset() -> Html {
    let seconds_state_handle = use_context::<UseReducerHandle<TickerState>>().unwrap();

    let onclick =
        { Callback::from(move |_| seconds_state_handle.dispatch(TickerStateAction::Reset)) };
    html! {
        <div class="column has-text-centered">
            <button class="button is-danger" {onclick}>
                <span class="icon-text">
                    <span class="icon">
                        <i class="material-icons">{"settings_backup_restore"}</i>
                    </span>
                    <span>{ "Reset" }</span>
                </span>
            </button>
        </div>
    }
}
