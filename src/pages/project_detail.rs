// src/pages/project_detail.rs
use crate::api;
use crate::types::Project;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
    project: Option<Project>,
    get_project_error: Option<Error>,
    get_project_loaded: bool,
}

pub struct ProjectDetail {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i32,
}

pub enum Msg {
    GetProject,
    GetProjectSuccess(Project),
    GetProjectError(Error),
}

impl Component for ProjectDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetProject);

        Self {
            props,
            state: State {
                project: None,
                get_project_error: None,
                get_project_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetProject => {
                let handler = self
                    .link
                    .callback(move |response: api::FetchResponse<Project>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(project) => Msg::GetProjectSuccess(project),
                            Err(err) => Msg::GetProjectError(err),
                        }
                    });

                self.task = Some(api::get_project(self.props.id, handler));
                true
            }
            Msg::GetProjectSuccess(project) => {
                self.state.project = Some(project);
                self.state.get_project_loaded = true;
                true
            }
            Msg::GetProjectError(error) => {
                self.state.get_project_error = Some(error);
                self.state.get_project_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(ref project) = self.state.project {
            html! {
                <div class="project_detail_container">
                    <img class="project_detail_image" src={&project.media}/>
                    <div class="project_card_name">{&project.title}</div>
                    <div style="margin: 10px 0; line-height: 24px;">{&project.description}</div>
                    <div class="project_card_price">{&project.year}</div>
                </div>
            }
        } else if !self.state.get_project_loaded {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else {
            html! {
                <div>
                    <span>{"Error loading project! :("}</span>
                </div>
            }
        }
    }
}