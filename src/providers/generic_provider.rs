use std::rc::Rc;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct GenericReducible<T: Default + Clone + PartialEq + 'static> {
    pub inner: T,
}

impl<T: Default + Clone + PartialEq + 'static> GenericReducible<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Default + Clone + PartialEq + 'static> Default for GenericReducible<T> {
    fn default() -> Self {
        Self {
            inner: T::default(),
        }
    }
}

impl<T: Default + Clone + PartialEq + 'static> Reducible for GenericReducible<T> {
    /// Reducer Action Type
    type Action = T;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self { inner: action }.into()
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn MessageGenericProvider<T: Default + Clone + PartialEq + 'static>(
    props: &MessageProviderProps,
) -> Html {
    let msg = use_reducer(|| GenericReducible::new(T::default()));
    html! {
        <ContextProvider<UseReducerHandle<GenericReducible<T>>> context={msg}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<GenericReducible<T>>>>
    }
}

#[function_component]
pub fn MessageReducibleProvider<T: Reducible + Default + Clone + PartialEq + 'static>(
    props: &MessageProviderProps,
) -> Html {
    let msg = use_reducer(|| T::default());
    html! {
        <ContextProvider<UseReducerHandle<T>> context={msg}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<T>>>
    }
}
