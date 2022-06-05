use core::fmt;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};
use yew::prelude::*;

use crate::providers::util::MessageProviderProps;

use super::timer::TimerState;

#[derive(PartialEq)]
pub struct GenericReducible<T: Default + Display + PartialEq + 'static> {
    inner: T,
}

impl<T: Default + Display + PartialEq + 'static> GenericReducible<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Default + Display + PartialEq + 'static> Default for GenericReducible<T> {
    fn default() -> Self {
        Self {
            inner: T::default(),
        }
    }
}

impl<T: Default + Display + PartialEq + 'static> Reducible for GenericReducible<T> {
    /// Reducer Action Type
    type Action = T;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self { inner: action }.into()
    }
}

impl<T: Default + Display + PartialEq + 'static> Display for GenericReducible<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

// TODO: generalize

// pub type MessageGenericContext = UseReducerHandle<GenericReducible<TimerState>>;

#[function_component]
pub fn MessageGenericProvider<T: Default + Display + PartialEq + 'static>(
    props: &MessageProviderProps,
) -> Html {
    let msg = use_reducer(|| GenericReducible::new(T::default()));
    html! {
        <ContextProvider<UseReducerHandle<GenericReducible<T>>> context={msg}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<GenericReducible<T>>>>
    }
}
