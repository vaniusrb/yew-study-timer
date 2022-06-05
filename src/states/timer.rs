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

// #[derive(Debug, Clone, PartialEq)]
// pub struct TimerState {
//     pub inner: TimerStateAction,
// }

// impl Default for TimerState {
//     fn default() -> Self {
//         Self {
//             inner: TimerStateAction::Stop,
//         }
//     }
// }

// impl TimerState {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// impl Display for TimerState {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{:?}", self.inner)
//     }
// }

// impl Reducible for TimerState {
//     /// Reducer Action Type
//     type Action = TimerStateAction;

//     /// Reducer Function
//     fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
//         TimerState { inner: action }.into()
//     }
// }
