use crate::{
    providers::generic_provider::GenericReducible,
    states::{
        start_stop::TimerStateAction,
        ticker::{TickerState, TickerStateAction},
    },
};
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[function_component]
pub fn Clock() -> Html {
    let timer_state_handle =
        use_context::<UseReducerHandle<GenericReducible<TimerStateAction>>>().unwrap();

    let ticker_state_handle = use_context::<UseReducerHandle<TickerState>>().unwrap();

    let interval_handle = use_mut_ref(|| Option::<Interval>::None);
    {
        let ticker_state_handle = ticker_state_handle.clone();
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
                            let interval = Interval::new(100, move || {
                                ticker_state_handle.dispatch(TickerStateAction::Tick)
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
            <div class="box">
                {format!("{}", *ticker_state_handle)}
            </div>
        </div>
    }
}
