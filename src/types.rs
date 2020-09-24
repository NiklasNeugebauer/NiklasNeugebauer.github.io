// src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub year: String
}