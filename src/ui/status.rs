use crate::app::state::{AppState, AppView};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render_status_bar(f: &mut Frame, area: Rect, state: &AppState) {
    let mut spans = vec![];

    if state.blackarch_detected {
        spans.push(Span::styled(
            " BlackArch ● ",
            Style::default().fg(Color::Green),
        ));
    } else {
        spans.push(Span::styled(
            " BlackArch ○ ",
            Style::default().fg(Color::DarkGray),
        ));
    }

    spans.push(Span::styled(
        format!("{} indexed", state.tool_count),
        Style::default(),
    ));

    spans.push(Span::raw(" │ "));

    spans.push(Span::styled(
        format!("{} installed", state.installed_count),
        Style::default().fg(Color::Green),
    ));

    if state.blackarch_count > 0 {
        spans.push(Span::raw(" │ "));
        spans.push(Span::styled(
            format!("{} BlackArch", state.blackarch_count),
            Style::default().fg(Color::Magenta),
        ));
    }

    spans.push(Span::raw(" │ "));

    if let Some(ref msg) = state.status_message {
        spans.push(Span::styled(
            msg.as_str(),
            Style::default().fg(Color::Green),
        ));
    } else if let Some(ref msg) = state.error_message {
        spans.push(Span::styled(msg.as_str(), Style::default().fg(Color::Red)));
    } else {
        let view_name = match state.current_view {
            AppView::Dashboard => "Dashboard",
            AppView::Categories => "Categories",
            AppView::ToolList => "Tools",
            AppView::ToolDetail => "Detail",
            AppView::Search => "Search",
            AppView::Profiles => "Profiles",
            AppView::Help => "Help",
            AppView::InstallPreview => "Install",
            AppView::InstallOutput => "Output",
            AppView::RemovePreview => "Remove",
            AppView::ExecutableSelect => "Run",
        };
        spans.push(Span::styled(
            format!(" {} ", view_name),
            Style::default().fg(Color::DarkGray),
        ));
    }

    spans.push(Span::styled(
        " ? Help ",
        Style::default().fg(Color::DarkGray),
    ));

    let status = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::Rgb(30, 30, 30)).fg(Color::White));

    f.render_widget(status, area);
}
