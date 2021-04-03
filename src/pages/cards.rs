// src/pages/cards.rs

use yew::prelude::*;

use crate::components::CardCollection;
use crate::components::NavBar;

struct State {}

pub struct Cards {
    props: Props,
    state: State
}

#[derive(Properties, Clone)]
pub struct Props {
    pub kind: String
}

pub enum Msg {}

impl Component for Cards {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {  props: Props {kind: "idk".to_string()},
                state: State {},
            }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let project_cards = html! {
            <CardCollection/>
        };

        let navigation = html! {
            <NavBar/>
        };

        html! {<div>
         <NavBar/>
         <div class="stripe_anchor">
         <h4>{&self.props.kind}</h4>
         <CardCollection/>
         </div>
        </div>}
    }
}
