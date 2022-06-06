use crate::{
    providers::generic_provider::GenericReducible,
    states::{
        seconds::{SecondsState, SecondsStateAction},
        timer::TimerStateAction,
    },
};
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[function_component]
pub fn Clock() -> Html {
    let timer_state_handle =
        use_context::<UseReducerHandle<GenericReducible<TimerStateAction>>>().unwrap();

    let seconds_state_handle = use_context::<UseReducerHandle<SecondsState>>().unwrap();

    let interval_handle = use_mut_ref(|| Option::<Interval>::None);

    {
        let seconds_state_handle = seconds_state_handle.clone();
        let timer_state_handle_dep = timer_state_handle.clone();
        use_effect_with_deps(
            move |_| {
                let mut interval_opt = interval_handle.borrow_mut();
                match timer_state_handle_dep.inner {
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
            timer_state_handle,
        );
    }
    html! {
        <div class ="column has-text-centered" style="width: 100px;">
            <span>{*seconds_state_handle}</span>
            <span>{" secs"}</span>
        </div>
    }
}
