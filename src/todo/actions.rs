use crate::{todo::reset_list, utils::read_keyboard_input};

use super::models::Todo;

pub fn add_todo(todo_list: &mut Vec<Todo>) {
    println!("\nAdding a new To-Do...");

    let title = read_keyboard_input("Title");
    let description = read_keyboard_input("Description");

    let new_todo = Todo::new(title, Some(description));

    todo_list.push(new_todo);

    reset_list(todo_list);
}

pub fn view_todo(todo_list: &Vec<Todo>) {
    println!("todoList: {:?}", todo_list);
    todo!("Print a pretty list of todos with colors")
}

pub fn delete_todo() {
    println!("deleting todo");
    todo!("Show a list of all the todos with their id's and decide which to remove")
}

pub fn complete_todo() {
    println!("completing todo");
    todo!()
}
