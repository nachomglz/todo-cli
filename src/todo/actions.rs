use crate::utils::read_keyboard_input;

use super::models::Todo;

pub fn add_todo(todo_list: &mut Vec<Todo>) {
    println!("Create a new Todo!");

    let title = read_keyboard_input("Title");
    let description = read_keyboard_input("Description");
    let completed = false;

    let new_todo = Todo::new(title, Some(description));

    todo_list.push(new_todo);
}

pub fn view_todo(todo_list: &Vec<Todo>) {
    println!("todoList: {:?}", todo_list);
}

pub fn delete_todo() {
    println!("deleting todo");
    todo!()
}

pub fn complete_todo() {
    println!("completing todo");
    todo!()
}
