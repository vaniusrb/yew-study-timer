use crate::{
    components::{clock::Clock, start::Start, stop::Stop},
    providers::msg_ctx::MessageTimerStateProvider,
};
use yew::prelude::*;

#[function_component]
pub fn Timer() -> Html {
    html! {
        <MessageTimerStateProvider>
        <div>
            <Clock/ >
            <Start/ >
            <Stop/ >
        </div>
        </MessageTimerStateProvider>
    }
}
