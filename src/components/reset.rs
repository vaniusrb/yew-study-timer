use crate::{msg_ctx::MessageContext, states::timer::TimerStateAction};
use yew::prelude::*;

#[function_component]
pub fn Stop() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let onclick = {
        // let msg_ctx = msg_ctx;
        Callback::from(move |_| msg_ctx.dispatch(TimerStateAction::Stop))
    };

    // let message = msg_ctx.inner.to_owned();

    html! {
        <div>
            <button {onclick}>
                {"<"}
            </button>
        </div>
    }
}
