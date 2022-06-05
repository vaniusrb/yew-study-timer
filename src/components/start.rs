use crate::{providers::state_provider::MessageTimerStateContext, states::timer::TimerStateAction};
use yew::prelude::*;

#[function_component]
pub fn Start() -> Html {
    let timer_state_handle = use_context::<MessageTimerStateContext>().unwrap();
    let onclick = { Callback::from(move |_| timer_state_handle.dispatch(TimerStateAction::Start)) };
    html! {
        <div>
            <button class="button is-primary" {onclick}>
                <span class="icon-text">
                    <span class="icon">
                        <i class="material-icons">{"play_arrow"}</i>
                    </span>
                    <span>{ "Play" }</span>
                </span>
            </button>
        </div>
    }
}
