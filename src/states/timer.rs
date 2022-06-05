use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum TimerStateAction {
    Stop,
    Start,
}

impl Display for TimerStateAction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for TimerStateAction {
    fn default() -> Self {
        TimerStateAction::Stop
    }
}
