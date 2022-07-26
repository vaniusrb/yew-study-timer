use crate::{
    components::{clock::Clock, reset::Reset, start::Start, stop::Stop},
    providers::generic_provider::{MessageGenericProvider, MessageReducibleProvider},
    states::{start_stop::TimerStateAction, ticker::TickerState},
};
use yew::prelude::*;

#[function_component]
pub fn Timer() -> Html {
    html! {
        <div>
            <MessageGenericProvider<TimerStateAction>>
            <MessageReducibleProvider<TickerState>>
            <section class="container">
            <div class="box" style="width: 600px;">
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
            </MessageReducibleProvider<TickerState>>
            </MessageGenericProvider<TimerStateAction>>
        </div>
    }
}
