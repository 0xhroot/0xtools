use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub border: Style,
    pub border_focus: Style,
    pub title: Style,
    pub text: Style,
    pub text_dim: Style,
    pub highlight: Style,
    pub highlight_bg: Style,
    pub installed: Style,
    pub available: Style,
    pub error: Style,
    pub status_bar: Style,
    pub search: Style,
    pub category_header: Style,
    pub tool_name: Style,
    pub tool_desc: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_theme()
    }
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            name: "default".to_string(),
            border: Style::default().fg(Color::DarkGray),
            border_focus: Style::default().fg(Color::Cyan),
            title: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            text: Style::default().fg(Color::White),
            text_dim: Style::default().fg(Color::DarkGray),
            highlight: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            highlight_bg: Style::default().bg(Color::DarkGray),
            installed: Style::default().fg(Color::Green),
            available: Style::default().fg(Color::DarkGray),
            error: Style::default().fg(Color::Red),
            status_bar: Style::default().fg(Color::White).bg(Color::DarkGray),
            search: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            category_header: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            tool_name: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            tool_desc: Style::default().fg(Color::Gray),
        }
    }

    pub fn minimal_theme() -> Self {
        Self {
            name: "minimal".to_string(),
            border: Style::default().fg(Color::Gray),
            border_focus: Style::default().fg(Color::White),
            title: Style::default().add_modifier(Modifier::BOLD),
            text: Style::default(),
            text_dim: Style::default().fg(Color::Gray),
            highlight: Style::default().add_modifier(Modifier::BOLD),
            highlight_bg: Style::default().bg(Color::Gray),
            installed: Style::default().fg(Color::Green),
            available: Style::default().fg(Color::Gray),
            error: Style::default().fg(Color::Red),
            status_bar: Style::default().bg(Color::Gray),
            search: Style::default().add_modifier(Modifier::BOLD),
            category_header: Style::default().add_modifier(Modifier::BOLD),
            tool_name: Style::default().add_modifier(Modifier::BOLD),
            tool_desc: Style::default(),
        }
    }

    pub fn by_name(name: &str) -> Self {
        match name {
            "minimal" => Self::minimal_theme(),
            _ => Self::default_theme(),
        }
    }
}
