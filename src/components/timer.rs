use crate::{
    components::{clock::Clock, reset::Reset, start::Start, stop::Stop},
    providers::generic_provider::{MessageGenericProvider, MessageReducibleProvider},
    states::{seconds::SecondsState, timer::TimerStateAction},
};
use yew::prelude::*;

#[function_component]
pub fn Timer() -> Html {
    html! {
        <div>
            <MessageGenericProvider<TimerStateAction>>
            <MessageReducibleProvider<SecondsState>>
            <section class="container">
            <div class="box">
            <div class="features">
            <div class="container">
                <div class="columns is-centered is-vcentered">
                    <Clock/ >
                    <Start/ >
                    <Stop/ >
                    <Reset/ >
                </div>
            </div>
            </div>
            </div>
            </section>
            </MessageReducibleProvider<SecondsState>>
            </MessageGenericProvider<TimerStateAction>>
        </div>
    }
}
