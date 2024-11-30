use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Todo {
    id: Option<u32>,
    title: String,
    description: Option<String>,
    completed: bool,
}

impl Todo {
    pub fn new(id: Option<u32>, title: String, description: Option<String>) -> Self {
        Self {
            id,
            title,
            description,
            completed: false,
        }
    }
}
