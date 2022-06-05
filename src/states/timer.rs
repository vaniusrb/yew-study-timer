use core::fmt;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TimerStateAction {
    Stop,
    Start,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimerState {
    state: TimerStateAction,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            state: TimerStateAction::Stop,
        }
    }
}

impl TimerState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Display for TimerState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.state)
    }
}

impl Reducible for TimerState {
    /// Reducer Action Type
    type Action = TimerStateAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        TimerState { state: action }.into()
    }
}
