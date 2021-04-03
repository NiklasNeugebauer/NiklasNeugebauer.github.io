// src/pages/home.rs

use yew::prelude::*;

use crate::components::CardCollection;
use crate::components::NavBar;
use crate::components::Footer;

struct State {}

pub struct Home {
    state: State,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { state: State {} }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        html! {<div>
            <NavBar/>
            <div class="stripe_anchor">
                <h4>{ "Home" }</h4>
                <img class="image_centered" src="/img/Logo.svg"/>
            </div>
            <Footer/>
        </div>}
    }
}
