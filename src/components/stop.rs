use crate::{providers::state_provider::MessageTimerStateContext, states::timer::TimerStateAction};
use yew::prelude::*;

#[function_component]
pub fn Stop() -> Html {
    let timer_state_handle = use_context::<MessageTimerStateContext>().unwrap();
    let onclick = { Callback::from(move |_| timer_state_handle.dispatch(TimerStateAction::Stop)) };
    html! {
        <div>
            <button class="button is-warning" {onclick}>
                <span class="icon-text">
                    <span class="icon">
                        <i class="material-icons">{"stop"}</i>
                    </span>
                    <span>{ "Stop" }</span>
                </span>
            </button>
        </div>
    }
}
