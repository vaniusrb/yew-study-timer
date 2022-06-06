use crate::states::seconds::{SecondsState, SecondsStateAction};
use yew::prelude::*;

#[function_component]
pub fn Reset() -> Html {
    let seconds_state_handle = use_context::<UseReducerHandle<SecondsState>>().unwrap();

    let onclick =
        { Callback::from(move |_| seconds_state_handle.dispatch(SecondsStateAction::Reset)) };
    html! {
        <div>
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
