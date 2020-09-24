// src/pages/home.rs

use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::prelude::*;

use crate::types::Project;
use crate::api;
use crate::components::ProjectCard;

struct State {
    projects: Vec<Project>,
    get_projects_error: Option<Error>,
    get_projects_loaded: bool
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>
}

pub enum Msg {
    GetProjects,
    GetProjectsSuccess(Vec<Project>),
    GetProjectsError(Error)
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let projects: Vec<Project> = vec![];

        link.send_message(Msg::GetProjects);

        Self {
            state: State {
                projects,
                get_projects_error: None,
                get_projects_loaded: false
            },
            link,
            task: None
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetProjects => {
                self.state.get_projects_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Vec<Project>>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(projects) => Msg::GetProjectsSuccess(projects),
                                Err(err) => Msg::GetProjectsError(err)
                            }
                        });
                self.task = Some(api::get_projects(handler));
                true
            }

            Msg::GetProjectsSuccess(projects) => {
                self.state.projects = projects;
                self.state.get_projects_loaded = true;
                true
            }

            Msg::GetProjectsError(err) => {
                self.state.get_projects_error = Some(err);
                self.state.get_projects_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let projects: Vec<Html> = self
            .state
            .projects
            .iter()
            .map(|project: &Project| {
                html! {
                    <ProjectCard project={project}/>
                }
            })
            .collect();
        if !self.state.get_projects_loaded {
            html! { 
                <div class="loading_indication_container">
                    <img class="loading_indicator" src="/img/wasm_logo.png"/>
                    <div class="loading_indicator_text">{"Loading..."}</div>
                </div> 
            }
        }
        else if let Some(_) = self.state.get_projects_error {
            html! {
                <span>{"Error loading projects! :("}</span>
            }
        }
        else {
            html! { <div class="project_card_list">{projects}</div> }
        }
    }
}