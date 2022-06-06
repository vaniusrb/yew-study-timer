use crate::{providers::generic_provider::GenericReducible, states::timer::TimerStateAction};
use yew::prelude::*;

#[function_component]
pub fn Stop() -> Html {
    let timer_state_handle =
        use_context::<UseReducerHandle<GenericReducible<TimerStateAction>>>().unwrap();

    let onclick = { Callback::from(move |_| timer_state_handle.dispatch(TimerStateAction::Stop)) };
    html! {
        <div class="column has-text-centered">
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
