use todo_cli::cli::{parse_args, TodoAction};
use todo_cli::todo;
use todo_cli::utils::get_todo_list;

fn main() {
    let args = parse_args();

    let mut todo_list = get_todo_list();

    match args.action {
        TodoAction::Add => todo::add_todo(&mut todo_list),
        TodoAction::View => todo::view_todo(&todo_list),
        TodoAction::Complete => todo::complete_todo(),
        TodoAction::Delete => todo::delete_todo(),
    }

    // reset document
}
