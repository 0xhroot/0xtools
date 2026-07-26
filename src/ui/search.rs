use crate::app::state::AppState;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

pub fn render_search(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(area);

    let input = Paragraph::new(Line::from(vec![
        Span::styled(
            " > ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            &state.search_input,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("█", Style::default().fg(Color::Yellow)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(state.theme.border_focus)
            .title(" Search Tools "),
    );
    f.render_widget(input, chunks[0]);

    let hint = if state.search_input.is_empty() {
        "  Type to search across tool names, descriptions, categories, and tags"
    } else {
        "  Type to search  •  j/k: navigate  •  Enter: open  •  Esc: close"
    };
    let hint_widget = Paragraph::new(Line::from(Span::styled(
        hint,
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(hint_widget, chunks[1]);

    let mut items: Vec<ListItem> = Vec::new();
    let viewport_height = chunks[2].height as usize;

    for (idx, &tool_idx) in state.search_results.iter().enumerate() {
        if idx >= state.tool_list_state.offset
            && idx < state.tool_list_state.offset + viewport_height
        {
            if let Some(tool) = state.search_index.tool(tool_idx) {
                let is_selected = state.search_result_selected == idx;
                let style = if is_selected {
                    state.theme.highlight
                } else {
                    state.theme.text
                };

                let arrow = if is_selected { " > " } else { "   " };
                let status = if tool.installed { "● " } else { "○ " };

                let desc = if tool.short_description.len() > 45 {
                    format!("{}…", &tool.short_description[..44])
                } else {
                    tool.short_description.clone()
                };

                let line = Line::from(vec![
                    Span::styled(arrow, style),
                    Span::styled(
                        status,
                        if tool.installed {
                            state.theme.installed
                        } else {
                            state.theme.available
                        },
                    ),
                    Span::styled(
                        format!("  {:<22}", tool.name),
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
                    Span::styled(
                        format!("  [{:>10}]", tool.repository.name()),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]);

                items.push(ListItem::new(line));
            }
        }
    }

    let result_count = state.search_results.len();
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(state.theme.border)
            .title(format!(" Results ({}) ", result_count)),
    );

    f.render_widget(list, chunks[2]);
}
