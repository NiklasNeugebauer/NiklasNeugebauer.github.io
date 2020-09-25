// src/api.rs
use crate::types::Project;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_projects(callback: FetchCallback<Vec<Project>>) -> FetchTask {
    let req = Request::get("/projects/projects.json")
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}