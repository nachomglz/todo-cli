use serde::Serialize;
use serde_json;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let home_dir = env::var("HOME").expect("HOME environment variable not found/set");
    let home_path = Path::new(&home_dir);
    let file_path = home_path.join(".todo-cli").join("todo.json");

    let action = Args::parse().action;

    let contents = fs::read_to_string(&file_path).expect("The file could not be read");

    let mut todo_list: Vec<Todo> = serde_json::from_str(&contents).unwrap();

    match action {
        TodoAction::Add => add_todo(&mut todo_list),
        TodoAction::View => view_todo(&todo_list),
        TodoAction::Complete => complete_todo(),
        TodoAction::Delete => delete_todo(),
    }
}
