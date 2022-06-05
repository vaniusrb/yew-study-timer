use super::util::MessageProviderProps;
use crate::states::seconds::SecondsState;
use yew::prelude::*;

pub type MessageSecondsStateContext = UseReducerHandle<SecondsState>;

#[function_component]
pub fn MessageSecondsStateProvider(props: &MessageProviderProps) -> Html {
    let msg = use_reducer(SecondsState::default);
    html! {
        <ContextProvider<MessageSecondsStateContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<MessageSecondsStateContext>>
    }
}
