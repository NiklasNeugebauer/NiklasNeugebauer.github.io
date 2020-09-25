// src/components/project_card.rs

use crate::types::Project;
use yew::prelude::*;

pub struct ProjectCard {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub project: Project
}


impl Component for ProjectCard {
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
        html! {
            <div class="project_card_container">
              <img class="project_card_image" src={&self.props.project.preview}/>
              <div class="project_card_name">{&self.props.project.title}</div>
              <div class="project_card_year">{&self.props.project.year}</div>
            </div>
        }
    }
}