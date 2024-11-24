use clap::Parser;
use serde::Serialize;

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
    let action = Args::parse().action;

    match action {
        TodoAction::Add => add_todo(),
        TodoAction::View => view_todo(),
        TodoAction::Complete => complete_todo(),
        TodoAction::Delete => delete_todo(),
    }
}

fn add_todo() {
    todo!()
}

fn view_todo() {
    todo!()
}

fn delete_todo() {
    todo!()
}

fn complete_todo() {
    todo!()
}
