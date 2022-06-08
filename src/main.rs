pub mod components;
pub mod providers;
pub mod states;

use crate::components::timer::Timer;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <section class="hero is-primary">
                <div class="hero-body">
                    <p class="title">{"Timer"}</p>
                    <p class="subtitle">{"Seconds counter"}</p>
                </div>
            </section>
            <br/>
            <div class="columns is-mobile">
                <div class="column is-half is-offset-one-quarter"><Timer /></div>
            </div>
            <footer class="footer">
                <div class="content has-text-right">
                    <p><strong>{"Yew Timer"}</strong> {" powered by Rust"}</p>
                </div>
            </footer>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
