use serde::{Deserialize, Serialize};

use super::generate_unique_id;

#[derive(Deserialize, Debug, Serialize)]
pub struct Todo {
    id: Option<u32>,
    title: String,
    description: Option<String>,
    completed: bool,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: Some(generate_unique_id()),
            title,
            description,
            completed: false,
        }
    }
}
