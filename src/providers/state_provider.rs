use std::fmt::Display;

use super::util::MessageProviderProps;
use crate::states::{
    generic_reducible::GenericReducible,
    timer::{TimerState, TimerStateAction},
};
use yew::prelude::*;

pub type MessageTimerStateContext = UseReducerHandle<TimerState>;

#[function_component]
pub fn MessageTimerStateProvider(props: &MessageProviderProps) -> Html {
    let msg = use_reducer(|| TimerState {
        inner: TimerStateAction::Stop,
    });
    html! {
        <ContextProvider<MessageTimerStateContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<MessageTimerStateContext>>
    }
}
