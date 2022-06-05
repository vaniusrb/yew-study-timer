pub mod components;
pub mod providers;
pub mod states;
pub mod timer_test;

use crate::components::timer::Timer;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <Timer />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
