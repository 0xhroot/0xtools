use crate::app::state::AppState;
use crate::catalog::Category;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

pub fn render_dashboard(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);

    let title_text = vec![
        Span::styled(
            "  0xtools",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            "Cybersecurity Tool Browser",
            Style::default().fg(Color::DarkGray),
        ),
    ];
    let title = Paragraph::new(Line::from(title_text)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(state.theme.border_focus)
            .title(" Home "),
    );
    f.render_widget(title, chunks[0]);

    let all_categories = Category::all();
    let categories_with_tools: Vec<(Category, usize)> = all_categories
        .iter()
        .filter_map(|cat| {
            let count = state.search_index.count_in_category(cat);
            if count > 0 {
                Some((*cat, count))
            } else {
                None
            }
        })
        .collect();

    let mut items: Vec<ListItem> = Vec::new();

    for (visual_idx, (cat, count)) in categories_with_tools.iter().enumerate() {
        let is_selected = state.category_list_state.selected == visual_idx;

        let style = if is_selected {
            state.theme.highlight
        } else {
            state.theme.text
        };

        let arrow = if is_selected { "> " } else { "  " };

        let line = Line::from(vec![
            Span::styled(arrow, style),
            Span::styled(
                format!("{:<28}", cat.name()),
                if is_selected {
                    state.theme.highlight
                } else {
                    state.theme.text
                },
            ),
            Span::styled(
                format!("{:>6} tools", count),
                Style::default().fg(Color::DarkGray),
            ),
        ]);

        items.push(ListItem::new(line));
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Categories ")
            .border_style(state.theme.border),
    );

    f.render_widget(list, chunks[1]);
}
