use clap::Parser;

use super::TodoAction;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TodoArgs {
    pub action: TodoAction,
}
