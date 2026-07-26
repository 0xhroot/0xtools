use crate::app::state::AppState;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_help(f: &mut Frame, area: Rect, state: &AppState) {
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  0xtools — Keyboard Reference",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Navigation",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  j / Down       Move down"),
        Line::from("  k / Up         Move up"),
        Line::from("  h / Left       Go back"),
        Line::from("  l / Right      Enter / Open"),
        Line::from("  Enter          Select / Open"),
        Line::from("  Esc            Back / Close"),
        Line::from("  Tab            Next pane"),
        Line::from(""),
        Line::from(Span::styled(
            "  Actions",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  /  s           Open search"),
        Line::from("  f              Toggle favorite"),
        Line::from("  i              Install tool"),
        Line::from("  u              Uninstall tool"),
        Line::from("  r              Run executable"),
        Line::from("  p              Profiles"),
        Line::from("  n              Next category"),
        Line::from("  ?              This help screen"),
        Line::from("  q              Quit"),
        Line::from(""),
        Line::from(Span::styled(
            "  Search Syntax",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  Free text:     nmap scanner"),
        Line::from("  Category:      category:web"),
        Line::from("  Installed:     installed:true"),
        Line::from("  Favorite:      favorite:true"),
        Line::from("  Repo:          repo:blackarch"),
        Line::from("  Tag:           tag:scanner"),
        Line::from(""),
        Line::from(Span::styled(
            "  CLI Commands",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  0xtools                  Launch TUI"),
        Line::from("  0xtools search <query>   Search tools"),
        Line::from("  0xtools info <name>      Tool details"),
        Line::from("  0xtools categories       List categories"),
        Line::from("  0xtools list <cat>       Tools in category"),
        Line::from("  0xtools installed        Installed tools"),
        Line::from("  0xtools favorites        Favorite tools"),
        Line::from("  0xtools sync             Refresh database"),
        Line::from("  0xtools doctor           System check"),
        Line::from("  0xtools version          Version info"),
        Line::from("  0xtools profiles         List profiles"),
        Line::from(""),
        Line::from(Span::styled(
            "  Press Esc or q to return",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
    ];

    let help = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(state.theme.border_focus)
                .title(" Help (? to close) "),
        )
        .scroll((state.help_scroll, 0))
        .wrap(Wrap { trim: true });

    f.render_widget(help, area);
}
