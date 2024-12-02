pub mod actions;
pub mod models;

use std::{env, error::Error, path::Path};

pub use actions::*;
pub use models::*;

use rand;

use crate::utils::write_file;

pub fn generate_unique_id() -> u32 {
    rand::random::<u32>()
}

pub fn reset_list(todo_list: &[Todo]) -> Result<(), Box<dyn Error>> {
    let content = serde_json::to_string_pretty(todo_list).expect("Error deserializing todo list");

    let home_dir = env::var("HOME").expect("The $HOME variable is not set");
    let home_path = Path::new(&home_dir);
    let file_path = home_path.join(".todo-cli").join("todo.json");

    match write_file(&content, file_path) {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}
