use gloo_timers::callback::Interval;
use yew::prelude::*;

use crate::states::seconds::{SecondsState, SecondsStateAction};

// https://stackoverflow.com/questions/71270366/using-wasm-timer-in-yew-to-execute-callback-repeatedly
// https://yew.rs/docs/concepts/function-components/pre-defined-hooks

#[function_component]
pub fn TimerTest() -> Html {
    let seconds_state_handle = use_reducer(SecondsState::default);

    let interval = use_state(|| Option::<Interval>::None);

    use_effect_with_deps(
        {
            let seconds_state_handle = seconds_state_handle.clone();

            move |_| {
                // i intervals get out of scope they get dropped and destroyed
                let interval = Interval::new(1000, move || {
                    seconds_state_handle.dispatch(SecondsStateAction::Increment)
                });

                // interval.can

                // So we move it into the clean up function, rust will consider this still being used and wont drop it
                // then we just drop it ourselves in the cleanup
                move || drop(interval)
            }
        },
        (), // Only create the interval once per your component existence
    );

    html! {<h1>{*seconds_state_handle}{" seconds has passed since this component got rendered"}</h1>}
}
