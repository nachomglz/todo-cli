use serde_json;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::todo::Todo;

pub fn read_keyboard_input(input_name: &str) -> String {
    let mut buf = String::new();

    println!("\n{}: ", input_name);

    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");

    return buf.trim().to_string();
}

pub fn get_todo_list() -> Vec<Todo> {
    let home_dir = env::var("HOME").expect("The $HOME variable is not set");
    let home_path = Path::new(&home_dir);
    let file_path = home_path.join(".todo-cli").join("todo.json");

    let file_contents = fs::read_to_string(&file_path).expect("The file could not be read");

    serde_json::from_str::<Vec<Todo>>(&file_contents)
        .expect("The List could not be read, it might have some typo inside it.")
}

pub fn write_file(content: &str, path: PathBuf) -> std::io::Result<()> {
    fs::write(path, content)
}
