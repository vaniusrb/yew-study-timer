use crate::states::timer::TimerStateAction;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MessageTimerState {
    pub inner: TimerStateAction,
}

impl Reducible for MessageTimerState {
    type Action = TimerStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        MessageTimerState { inner: action }.into()
    }
}

pub type MessageTimerStateContext = UseReducerHandle<MessageTimerState>;

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn MessageTimerStateProvider(props: &MessageProviderProps) -> Html {
    let msg = use_reducer(|| MessageTimerState {
        inner: TimerStateAction::Stop,
    });
    html! {
        <ContextProvider<MessageTimerStateContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<MessageTimerStateContext>>
    }
}
