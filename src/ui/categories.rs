use crate::app::state::AppState;
use ratatui::layout::Rect;
use ratatui::Frame;

pub fn render_categories(f: &mut Frame, area: Rect, state: &AppState) {
    crate::ui::tools::render_tool_list(f, area, state);
}
