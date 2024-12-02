pub mod actions;
pub mod models;

pub use actions::*;
use clap::Parser;
pub use models::*;

pub fn parse_args() -> TodoArgs {
    // IMPROV: Add error handling to the args parse and command manual
    TodoArgs::parse()
}
