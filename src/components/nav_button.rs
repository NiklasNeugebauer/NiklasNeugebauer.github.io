// src/components/nav_button.rs

use crate::route::Route;
use crate::types::NavObject;
use yew::prelude::*;
use yew::html::*;
use yew_router::components::RouterAnchor;

pub struct NavButton {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub nav_object: NavObject,
}

impl Component for NavButton {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let link_href = yew::html::Href::from(&self.props.nav_object.link[..]);
        html! {
            <a href=link_href>
            <div class="nav_button">
                <div class="nav_button_text">{&self.props.nav_object.text}</div>
            </div>
            </a>
        }
    }
}
