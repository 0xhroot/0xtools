use crate::app::state::AppState;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

pub fn render_tool_list(f: &mut Frame, area: Rect, state: &AppState) {
    let title = match state.selected_category {
        Some(cat) => format!(" {} ", cat.name()),
        None => " All Tools ".to_string(),
    };

    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled(&title, state.theme.category_header),
        Span::styled(
            format!(" ({} tools)", state.visible_tools.len()),
            Style::default().fg(Color::DarkGray),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(state.theme.border_focus)
            .title(" Tools "),
    );
    f.render_widget(header, chunks[0]);

    let viewport_height = chunks[1].height as usize;
    let mut state_clone = state.tool_list_state.clone();
    state_clone.adjust_offset(viewport_height);

    let start = state_clone.offset;
    let end = (start + viewport_height).min(state.visible_tools.len());

    let mut items: Vec<ListItem> = Vec::new();
    for idx in start..end {
        let tool_idx = state.visible_tools[idx];
        if let Some(tool) = state.search_index.tool(tool_idx) {
            let is_selected = state_clone.selected == idx;
            let style = if is_selected {
                state.theme.highlight
            } else if tool.installed {
                state.theme.installed
            } else {
                state.theme.text
            };

            let arrow = if is_selected { " > " } else { "   " };
            let status = if tool.installed { "● " } else { "○ " };

            let fav = if state.favorites.is_favorite(&tool.name) {
                "★ "
            } else {
                "  "
            };

            let desc = if tool.short_description.len() > 50 {
                format!("{}…", &tool.short_description[..49])
            } else {
                tool.short_description.clone()
            };

            let line = Line::from(vec![
                Span::styled(arrow, style),
                Span::styled(
                    fav,
                    if state.favorites.is_favorite(&tool.name) {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    },
                ),
                Span::styled(
                    status,
                    if tool.installed {
                        state.theme.installed
                    } else {
                        state.theme.available
                    },
                ),
                Span::styled(
                    format!("  {:<20}", tool.name),
                    if is_selected {
                        state.theme.highlight
                    } else {
                        state.theme.tool_name
                    },
                ),
                Span::styled(
                    format!("  {}", desc),
                    if is_selected {
                        Style::default().fg(Color::White)
                    } else {
                        state.theme.tool_desc
                    },
                ),
            ]);

            items.push(ListItem::new(line));
        }
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(state.theme.border)
            .title(format!(
                " {} ({}/{}) ",
                title.trim(),
                state_clone.selected + 1,
                state.visible_tools.len()
            )),
    );

    f.render_widget(list, chunks[1]);
}
