use clap::Parser;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(clap::ValueEnum, Clone, Debug, Serialize, Default)]
enum TodoAction {
    Add,
    #[default]
    View,
    Complete,
    Delete,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    action: TodoAction,
}

fn main() {
    let home_dir = env::var("HOME").expect("HOME environment variable not found/set");
    let home_path = Path::new(&home_dir);
    let file_path = home_path.join(".todo-cli").join("todo.json");

    let action = Args::parse().action;

    let contents = fs::read_to_string(&file_path).expect("The file could not be read");

    // TODO: Parse the contents to an object using Serde

    match action {
        TodoAction::Add => add_todo(),
        TodoAction::View => view_todo(),
        TodoAction::Complete => complete_todo(),
        TodoAction::Delete => delete_todo(),
    }
}

fn add_todo() {
    println!("adding todo");
    todo!()
}

fn view_todo() {
    println!("viewing todo");
    todo!()
}

fn delete_todo() {
    println!("deleting todo");
    todo!()
}

fn complete_todo() {
    println!("completing todo");
    todo!()
}
