use core::fmt;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecondsStateAction {
    Increment,
    Reset,
}

impl Default for SecondsStateAction {
    fn default() -> Self {
        Self::Reset
    }
}

impl Display for SecondsStateAction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct SecondsState {
    seconds: usize,
}

impl Display for SecondsState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.seconds)
    }
}

impl Reducible for SecondsState {
    type Action = SecondsStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SecondsStateAction::Increment => Self {
                seconds: self.seconds + 1,
            },
            SecondsStateAction::Reset => Self { seconds: 0 },
        }
        .into()
    }
}
