#[derive(clap::ValueEnum, Clone, Debug, Serialize, Default)]
pub enum TodoAction {
    Add,
    #[default]
    View,
    Complete,
    Delete,
}
