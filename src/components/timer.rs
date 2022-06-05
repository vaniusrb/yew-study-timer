use crate::{
    components::{clock::Clock, reset::Reset, start::Start, stop::Stop},
    providers::{
        seconds_provider::MessageSecondsStateProvider, state_provider::MessageTimerStateProvider,
    },
    states::{generic_reducible::MessageGenericProvider, timer::TimerStateAction},
};
use yew::prelude::*;

#[function_component]
pub fn Timer() -> Html {
    html! {
        <MessageGenericProvider<TimerStateAction>>
        <MessageTimerStateProvider>
        <MessageSecondsStateProvider>
        <div class="box" style="width: 500px;">
            <div class="columns is-vcentered">
                <div class="column"><Clock/ ></div>
                <div class="column"><Start/ ></div>
                <div class="column"><Stop/ ></div>
                <div class="column"><Reset/ ></div>
            </div>
        </div>
        </MessageSecondsStateProvider>
        </MessageTimerStateProvider>
        </MessageGenericProvider<TimerStateAction>>
    }
}
