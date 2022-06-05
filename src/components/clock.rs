use crate::{
    providers::msg_ctx::MessageTimerStateContext,
    states::{
        seconds::{SecondsState, SecondsStateAction},
        timer::TimerStateAction,
    },
};
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[function_component]
pub fn Clock() -> Html {
    let msg_ctx = use_context::<MessageTimerStateContext>().unwrap();

    let seconds_state_handle = use_reducer(SecondsState::default);
    let interval_handle = use_mut_ref(|| Option::<Interval>::None);

    {
        let seconds_state_handle = seconds_state_handle.clone();
        let msg_ctx_dep = msg_ctx.clone();
        use_effect_with_deps(
            move |_| {
                let mut interval_opt = interval_handle.borrow_mut();
                match msg_ctx_dep.inner {
                    TimerStateAction::Stop => {
                        if let Some(interval) = (*interval_opt).take() {
                            interval.cancel();
                            *interval_opt = None;
                        }
                    }
                    TimerStateAction::Start => {
                        if interval_opt.is_none() {
                            let interval = Interval::new(1000, move || {
                                seconds_state_handle.dispatch(SecondsStateAction::Increment)
                            });
                            *interval_opt = Some(interval);
                        }
                    }
                };
                || ()
            },
            msg_ctx, // Only create the interval once per your component existence
        );
    }

    html! {<h1>{*seconds_state_handle}{" seconds"}</h1>}
    // html! {
    //     { "00:00" }
    // }
}
