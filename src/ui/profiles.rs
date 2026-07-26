use crate::app::state::AppState;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_profiles(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]).split(area);

    let mut items: Vec<ListItem> = Vec::new();
    for (idx, profile) in state.profiles.iter().enumerate() {
        let is_selected = state.selected_profile == Some(idx);
        let style = if is_selected {
            state.theme.highlight
        } else {
            state.theme.text
        };

        let arrow = if is_selected { "> " } else { "  " };

        let line = Line::from(vec![
            Span::styled(arrow, style),
            Span::styled(
                profile.name.clone(),
                if is_selected {
                    state.theme.highlight
                } else {
                    state.theme.tool_name
                },
            ),
        ]);
        items.push(ListItem::new(line));
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(state.theme.border_focus)
            .title(" Profiles "),
    );
    f.render_widget(list, chunks[0]);

    if let Some(sel) = state.selected_profile {
        if let Some(profile) = state.profiles.get(sel) {
            let mut lines: Vec<Line> = Vec::new();
            lines.push(Line::from(Span::styled(
                &profile.name,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                &profile.description,
                state.theme.text,
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("Packages ({})", profile.packages.len()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));
            for pkg in &profile.packages {
                let installed = state
                    .search_index
                    .tool_by_name(pkg)
                    .map(|t| t.installed)
                    .unwrap_or(false);
                let status = if installed { "●" } else { "○" };
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {} ", status),
                        if installed {
                            state.theme.installed
                        } else {
                            state.theme.available
                        },
                    ),
                    Span::raw(pkg.clone()),
                ]));
            }

            let detail = Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(state.theme.border)
                        .title(" Profile Details "),
                )
                .wrap(Wrap { trim: true });
            f.render_widget(detail, chunks[1]);
        }
    } else {
        let empty = Paragraph::new("Select a profile").block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(state.theme.border)
                .title(" Profile Details "),
        );
        f.render_widget(empty, chunks[1]);
    }
}
