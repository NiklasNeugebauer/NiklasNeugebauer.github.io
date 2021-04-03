// src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub brief: String,
    pub description: String,
    pub preview: String,
    pub media: String,
    pub year: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NavObject {
    pub text: String,
    pub link: String
}
