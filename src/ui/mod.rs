pub mod categories;
pub mod dashboard;
pub mod details;
pub mod dialog;
pub mod help;
pub mod profiles;
pub mod search;
pub mod status;
pub mod tools;

pub use dashboard::render_dashboard;
pub use details::render_detail;
pub use dialog::{render_install_output, render_install_preview, render_remove_preview};
pub use help::render_help;
pub use profiles::render_profiles;
pub use search::render_search;
pub use status::render_status_bar;
pub use tools::render_tool_list;
