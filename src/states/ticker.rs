use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TickerStateAction {
    Tick,
    Reset,
}

impl Default for TickerStateAction {
    fn default() -> Self {
        Self::Reset
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TickerState {
    begin: Option<instant::Instant>,
    time: String,
}

impl Default for TickerState {
    fn default() -> Self {
        let begin = None;
        Self {
            begin,
            time: fmt_elapsed(begin),
        }
    }
}

fn fmt_elapsed(begin: Option<instant::Instant>) -> String {
    let (hours, minutes, seconds, mili) = match begin {
        Some(begin) => {
            let duration = begin.elapsed();
            let mili = duration.as_millis() % 1000;
            let seconds = duration.as_secs() % 60;
            let minutes = (duration.as_secs() / 60) % 60;
            let hours = (duration.as_secs() / 60) / 60;
            (hours, minutes, seconds, mili)
        }
        None => (0, 0, 0, 0),
    };
    format!("{hours}:{minutes:02}:{seconds:02},{mili:03}")
}

impl Display for TickerState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.time)
    }
}

impl Reducible for TickerState {
    type Action = TickerStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        log::info!("Dentro {action:?}");
        let begin = match action {
            TickerStateAction::Tick => Some(self.begin.unwrap_or_else(instant::Instant::now)),
            TickerStateAction::Reset => None,
        };
        Self {
            begin,
            time: fmt_elapsed(begin),
        }
        .into()
    }
}
