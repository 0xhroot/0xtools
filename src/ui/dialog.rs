use crate::app::state::AppState;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_install_preview(f: &mut Frame, area: Rect, state: &AppState) {
    match &state.install_preview {
        None => {
            let msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Installing package...",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Please wait while the package is being installed.",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(
                    "  You may be prompted for your sudo password.",
                    Style::default().fg(Color::DarkGray),
                )),
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(" Installing ")
                    .title_alignment(Alignment::Center),
            );
            f.render_widget(msg, area);
        }
        Some(preview) => {
            let area = centered_rect(60, 30, area);

            let mut lines: Vec<Line> = vec![
                Line::from(""),
                Line::from(Span::styled(
                    format!("  {}", preview.package),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("    Repository   ", Style::default().fg(Color::DarkGray)),
                    Span::raw(&preview.repository),
                ]),
                Line::from(vec![
                    Span::styled("    Version      ", Style::default().fg(Color::DarkGray)),
                    Span::raw(&preview.version),
                ]),
                Line::from(""),
            ];

            let new_deps: Vec<&String> = preview
                .dependencies
                .iter()
                .filter(|d| !preview.already_installed_deps.contains(d))
                .collect();
            let already_count = preview.already_installed_deps.len();

            if !preview.dependencies.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("  Dependencies ({} to install)", new_deps.len()),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(""));
                for dep in new_deps.iter().take(12) {
                    lines.push(Line::from(vec![
                        Span::styled("    + ", Style::default().fg(Color::Yellow)),
                        Span::raw(dep.to_string()),
                    ]));
                }
                if new_deps.len() > 12 {
                    lines.push(Line::from(Span::styled(
                        format!("    ... and {} more", new_deps.len() - 12),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
                if already_count > 0 {
                    lines.push(Line::from(Span::styled(
                        format!("    ({} already installed)", already_count),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
            } else {
                lines.push(Line::from(Span::styled(
                    "  No additional dependencies needed",
                    Style::default().fg(Color::DarkGray),
                )));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "    pacman -S will install the package",
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(Span::styled(
                "    and resolve all dependencies automatically.",
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "    Enter: install  •  Esc: cancel",
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(""));

            let dialog = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(" Confirm Installation ")
                    .title_alignment(Alignment::Center),
            );
            f.render_widget(dialog, area);
        }
    }
}

pub fn render_remove_preview(f: &mut Frame, area: Rect, state: &AppState) {
    match &state.remove_preview {
        None => {
            let msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Removing package...",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Please wait while the package is being removed.",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(
                    "  You may be prompted for your sudo password.",
                    Style::default().fg(Color::DarkGray),
                )),
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red))
                    .title(" Removing ")
                    .title_alignment(Alignment::Center),
            );
            f.render_widget(msg, area);
        }
        Some(preview) => {
            let area = centered_rect(60, 20, area);

            let lines: Vec<Line> = vec![
                Line::from(""),
                Line::from(Span::styled(
                    format!("  {}", preview.package),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  This will remove the package from your system.",
                    Style::default().fg(Color::Red),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "    Enter: remove  •  Esc: cancel",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
            ];

            let dialog = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red))
                    .title(" Confirm Removal ")
                    .title_alignment(Alignment::Center),
            );
            f.render_widget(dialog, area);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

pub fn render_install_output(f: &mut Frame, area: Rect, state: &AppState) {
    let output_text = match &state.install_output {
        Some(text) => text.as_str(),
        None => "",
    };

    let lines: Vec<Line> = output_text
        .lines()
        .map(|l| Line::from(Span::raw(l.to_string())))
        .collect();

    let scroll = state.install_output_scroll as usize;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .title(" Pacman Output ")
        .title_alignment(Alignment::Center);

    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((scroll as u16, 0));

    f.render_widget(paragraph, area);

    let help_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    let help = Paragraph::new(Line::from(Span::styled(
        " j/k: scroll  Esc/Enter: close",
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(help, help_area);
}
