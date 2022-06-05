use crate::{
    providers::seconds_provider::MessageSecondsStateContext, states::seconds::SecondsStateAction,
};
use yew::prelude::*;

#[function_component]
pub fn Reset() -> Html {
    let msg_ctx = use_context::<MessageSecondsStateContext>().unwrap();
    let onclick = { Callback::from(move |_| msg_ctx.dispatch(SecondsStateAction::Reset)) };
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
