// src/route.rs
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/project/{id}"]
    ProjectDetail(i32),
    #[to = "/"]
    HomePage,
}