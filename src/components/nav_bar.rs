// src/pages/nav_bar.rs

use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

use crate::api;
use crate::components::NavButton;
use crate::types::NavObject;

struct State {
    nav_objects: Vec<NavObject>,
    get_objects_error: Option<Error>,
    get_objects_loaded: bool,
}

pub struct NavBar {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    GetObjects,
    GetObjectsSuccess(Vec<NavObject>),
    GetObjectsError(Error),
}

impl Component for NavBar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let nav_objects: Vec<NavObject> = vec![];

        link.send_message(Msg::GetObjects);

        Self {
            state: State {
                nav_objects,
                get_objects_error: None,
                get_objects_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetObjects => {
                self.state.get_objects_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Vec<NavObject>>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(objects) => Msg::GetObjectsSuccess(objects),
                                Err(err) => Msg::GetObjectsError(err),
                            }
                        });
                self.task = Some(api::get_nav_objects(handler));
                true
            }

            Msg::GetObjectsSuccess(nav_objects) => {
                self.state.nav_objects = nav_objects;
                self.state.get_objects_loaded = true;
                true
            }

            Msg::GetObjectsError(err) => {
                self.state.get_objects_error = Some(err);
                self.state.get_objects_loaded = true;

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
            .nav_objects
            .iter()
            .map(|nav_object: &NavObject| {
                html! {
                    <NavButton nav_object={nav_object}/>
                }
            })
            .collect();

        if !self.state.get_objects_loaded {
            html! {
                <div class="loading_indication_container">
                    <img class="loading_indicator" src="/img/wasm_logo.png"/>
                    <div class="loading_indicator_text">{"Loading..."}</div>
                </div>
            }
        } else if let Some(_) = self.state.get_objects_error {
            html! {
                <span>{"Error loading navigation bar! This is bad :("}</span>
            }
        } else {
            html! { <div class="nav_bar">{projects}</div> }
        }
    }
}
