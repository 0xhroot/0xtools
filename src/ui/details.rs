use crate::app::state::AppState;
use crate::catalog::tool_knowledge::get_knowledge;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_detail(f: &mut Frame, area: Rect, state: &AppState) {
    let Some(tool) = state.current_tool() else {
        let msg = Paragraph::new("No tool selected").block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(state.theme.border)
                .title(" Error "),
        );
        f.render_widget(msg, area);
        return;
    };

    let chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(area);

    let mut lines: Vec<Line> = Vec::new();

    // ── Header ──
    lines.push(Line::from(vec![
        Span::styled(
            &tool.name,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            &tool.available_version,
            Style::default().fg(Color::DarkGray),
        ),
        Span::raw("  "),
        Span::styled(
            if tool.installed {
                "● INSTALLED"
            } else {
                "○ Available"
            },
            if tool.installed {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
        if state.favorites.is_favorite(&tool.name) {
            Span::styled("  ★ Favorite", Style::default().fg(Color::Yellow))
        } else {
            Span::raw("")
        },
    ]));

    lines.push(Line::from(""));

    // ── Short description ──
    if !tool.short_description.is_empty() {
        for line_text in wrap_text(&tool.short_description, 80) {
            lines.push(Line::from(Span::styled(
                format!("  {}", line_text),
                Style::default().fg(Color::White),
            )));
        }
        lines.push(Line::from(""));
    }

    // ── About This Tool (rich knowledge) ──
    let knowledge = get_knowledge(tool);

    lines.push(Line::from(Span::styled(
        "  ════════════════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        "  ABOUT THIS TOOL",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  ════════════════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    // What it does
    lines.push(Line::from(Span::styled(
        "    What It Does",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )));
    for line_text in wrap_text(&knowledge.what_it_does, 72) {
        lines.push(Line::from(Span::styled(
            format!("      {}", line_text),
            state.theme.text,
        )));
    }
    lines.push(Line::from(""));

    // How it works
    lines.push(Line::from(Span::styled(
        "    How It Works",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )));
    for line_text in wrap_text(&knowledge.how_it_works, 72) {
        lines.push(Line::from(Span::styled(
            format!("      {}", line_text),
            state.theme.text,
        )));
    }
    lines.push(Line::from(""));

    // Difficulty
    lines.push(Line::from(vec![
        Span::styled("    Difficulty    ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            knowledge.difficulty.to_string(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    // Attack types
    if !knowledge.attack_types.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("    Attack Type   ", Style::default().fg(Color::DarkGray)),
            Span::raw(knowledge.attack_types.join(" · ")),
        ]));
    }

    // Protocols
    if !knowledge.protocols.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("    Protocols     ", Style::default().fg(Color::DarkGray)),
            Span::raw(knowledge.protocols.join(", ")),
        ]));
    }
    lines.push(Line::from(""));

    // Use Cases
    if !knowledge.use_cases.is_empty() {
        lines.push(Line::from(Span::styled(
            "    Use Cases",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        for uc in &knowledge.use_cases {
            lines.push(Line::from(vec![
                Span::styled("      ● ", Style::default().fg(Color::Green)),
                Span::raw(uc.clone()),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Key Features
    if !knowledge.key_features.is_empty() {
        lines.push(Line::from(Span::styled(
            "    Key Features",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        for feat in &knowledge.key_features {
            lines.push(Line::from(vec![
                Span::styled("      ✓ ", Style::default().fg(Color::Green)),
                Span::raw(feat.clone()),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Targets
    if !knowledge.targets.is_empty() {
        lines.push(Line::from(Span::styled(
            "    Targets",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        for t in &knowledge.targets {
            lines.push(Line::from(vec![
                Span::styled("      ● ", Style::default().fg(Color::DarkGray)),
                Span::raw(t.clone()),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Strengths
    if !knowledge.strengths.is_empty() {
        lines.push(Line::from(Span::styled(
            "    Strengths",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        for s in &knowledge.strengths {
            lines.push(Line::from(vec![
                Span::styled("      + ", Style::default().fg(Color::Green)),
                Span::raw(s.clone()),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Limitations
    if !knowledge.limitations.is_empty() {
        lines.push(Line::from(Span::styled(
            "    Limitations",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        for l in &knowledge.limitations {
            lines.push(Line::from(vec![
                Span::styled("      - ", Style::default().fg(Color::Yellow)),
                Span::raw(l.clone()),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Alternatives
    if !knowledge.alternatives.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("    Alternatives  ", Style::default().fg(Color::DarkGray)),
            Span::raw(knowledge.alternatives.join(" · ")),
        ]));
    }

    // Best Practices
    if !knowledge.best_practices.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "    Best Practices",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        for bp in &knowledge.best_practices {
            lines.push(Line::from(vec![
                Span::styled("      → ", Style::default().fg(Color::Cyan)),
                Span::raw(bp.clone()),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Typical Workflow
    if !knowledge.typical_workflow.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "    Typical Workflow",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        for step in &knowledge.typical_workflow {
            lines.push(Line::from(Span::styled(
                format!("      {}", step),
                Style::default().fg(Color::White),
            )));
        }
        lines.push(Line::from(""));
    }

    // ── Classification ──
    lines.push(Line::from(Span::styled(
        "  ════════════════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        "  CLASSIFICATION",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  ════════════════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    let cats: Vec<String> = tool
        .categories
        .iter()
        .map(|c| c.name().to_string())
        .collect();
    lines.push(Line::from(vec![
        Span::styled("    Category    ", Style::default().fg(Color::DarkGray)),
        Span::raw(cats.join(", ")),
    ]));

    if !tool.tags.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("    Tags        ", Style::default().fg(Color::DarkGray)),
            Span::raw(tool.tags.join(" · ")),
        ]));
    }

    if !tool.groups.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("    Groups      ", Style::default().fg(Color::DarkGray)),
            Span::raw(tool.groups.join(", ")),
        ]));
    }

    // ── Package Information ──
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  ════════════════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        "  PACKAGE INFORMATION",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  ════════════════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::styled("    Repository    ", Style::default().fg(Color::DarkGray)),
        Span::raw(tool.repository.name()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("    Version       ", Style::default().fg(Color::DarkGray)),
        Span::raw(&tool.available_version),
    ]));
    lines.push(Line::from(vec![
        Span::styled("    Status        ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            tool.status_text().to_string(),
            if tool.installed {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]));
    if let Some(ref ver) = tool.installed_version {
        lines.push(Line::from(vec![
            Span::styled("    Installed Ver ", Style::default().fg(Color::DarkGray)),
            Span::raw(ver.clone()),
        ]));
    }

    if let Some(ref arch) = tool.arch {
        lines.push(Line::from(vec![
            Span::styled("    Architecture  ", Style::default().fg(Color::DarkGray)),
            Span::raw(arch.clone()),
        ]));
    }

    if let Some(ref packager) = tool.packager {
        lines.push(Line::from(vec![
            Span::styled("    Packager      ", Style::default().fg(Color::DarkGray)),
            Span::raw(packager.clone()),
        ]));
    }

    if !tool.licenses.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("    License       ", Style::default().fg(Color::DarkGray)),
            Span::raw(tool.licenses.join(", ")),
        ]));
    }

    if let Some(ref homepage) = tool.homepage {
        lines.push(Line::from(vec![
            Span::styled("    Homepage      ", Style::default().fg(Color::DarkGray)),
            Span::styled(homepage.clone(), Style::default().fg(Color::Blue)),
        ]));
    }

    if let Some(size) = tool.download_size {
        lines.push(Line::from(vec![
            Span::styled("    Download Size ", Style::default().fg(Color::DarkGray)),
            Span::raw(format_size(size)),
        ]));
    }

    if let Some(size) = tool.installed_size {
        lines.push(Line::from(vec![
            Span::styled("    Installed Size", Style::default().fg(Color::DarkGray)),
            Span::raw(format_size(size)),
        ]));
    }

    if let Some(date) = tool.build_date {
        lines.push(Line::from(vec![
            Span::styled("    Build Date    ", Style::default().fg(Color::DarkGray)),
            Span::raw(format_timestamp(date)),
        ]));
    }

    if let Some(date) = tool.install_date {
        lines.push(Line::from(vec![
            Span::styled("    Installed On  ", Style::default().fg(Color::DarkGray)),
            Span::raw(format_timestamp(date)),
        ]));
    }

    if let Some(ref filename) = tool.filename {
        lines.push(Line::from(vec![
            Span::styled("    Filename      ", Style::default().fg(Color::DarkGray)),
            Span::raw(filename.clone()),
        ]));
    }

    lines.push(Line::from(vec![
        Span::styled("    Source        ", Style::default().fg(Color::DarkGray)),
        Span::raw(format!("{:?}", tool.metadata_source)),
    ]));

    // ── Executables ──
    if !tool.executables.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            "  EXECUTABLES",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
        for exec in &tool.executables {
            lines.push(Line::from(vec![
                Span::styled("    → ", Style::default().fg(Color::Green)),
                Span::styled(exec.clone(), Style::default().fg(Color::Green)),
            ]));
        }
    }

    // ── Dependencies ──
    if !tool.dependencies.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            format!("  DEPENDENCIES ({})", tool.dependencies.len()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
        for dep in tool.dependencies.iter().take(25) {
            lines.push(Line::from(vec![
                Span::styled("    • ", Style::default().fg(Color::DarkGray)),
                Span::raw(dep.clone()),
            ]));
        }
        if tool.dependencies.len() > 25 {
            lines.push(Line::from(Span::styled(
                format!("    … and {} more", tool.dependencies.len() - 25),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    // ── Optional Dependencies ──
    if !tool.optional_dependencies.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            format!(
                "  OPTIONAL DEPENDENCIES ({})",
                tool.optional_dependencies.len()
            ),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
        for dep in tool.optional_dependencies.iter().take(15) {
            lines.push(Line::from(vec![
                Span::styled("    • ", Style::default().fg(Color::DarkGray)),
                Span::raw(dep.clone()),
            ]));
        }
        if tool.optional_dependencies.len() > 15 {
            lines.push(Line::from(Span::styled(
                format!("    … and {} more", tool.optional_dependencies.len() - 15),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    // ── Conflicts ──
    if !tool.conflicts.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  CONFLICTS ({})", tool.conflicts.len()),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
        for c in tool.conflicts.iter().take(10) {
            lines.push(Line::from(vec![
                Span::styled("    ⚠ ", Style::default().fg(Color::Yellow)),
                Span::raw(c.clone()),
            ]));
        }
    }

    // ── Related Tools ──
    if !tool.related.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            "  RELATED TOOLS",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
        for r in &tool.related {
            lines.push(Line::from(vec![
                Span::styled("    ● ", Style::default().fg(Color::DarkGray)),
                Span::raw(r.clone()),
            ]));
        }
    }

    // ── Usage Examples ──
    let examples = generate_usage_examples(tool);
    if !examples.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            "  USAGE EXAMPLES",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ════════════════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )));
        for example in &examples {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("    $ ", Style::default().fg(Color::Green)),
                Span::styled(
                    example.cmd.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            if !example.desc.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("      ", Style::default()),
                    Span::styled(example.desc.clone(), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
    }

    let scroll = state.detail_scroll as usize;
    let detail = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(state.theme.border_focus)
                .title(format!(" {} ", tool.name)),
        )
        .scroll((scroll as u16, 0))
        .wrap(Wrap { trim: true });

    f.render_widget(detail, chunks[0]);

    let footer = if tool.installed {
        Line::from(vec![
            Span::styled(" [i] Install ", Style::default().fg(Color::DarkGray)),
            Span::styled(" [u] Remove ", Style::default().fg(Color::DarkGray)),
            Span::styled(" [f] Favorite ", Style::default().fg(Color::DarkGray)),
            Span::styled(" [Esc] Back ", Style::default().fg(Color::DarkGray)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" [i] Install ", Style::default().fg(Color::DarkGray)),
            Span::styled(" [f] Favorite ", Style::default().fg(Color::DarkGray)),
            Span::styled(" [Esc] Back ", Style::default().fg(Color::DarkGray)),
        ])
    };

    let footer_widget = Paragraph::new(footer).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(state.theme.border),
    );

    f.render_widget(footer_widget, chunks[1]);
}

fn format_size(bytes: i64) -> String {
    if bytes < 0 {
        return "N/A".into();
    }
    let bytes = bytes as f64;
    if bytes < 1024.0 {
        format!("{} B", bytes)
    } else if bytes < 1024.0 * 1024.0 {
        format!("{:.1} KiB", bytes / 1024.0)
    } else if bytes < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1} MiB", bytes / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GiB", bytes / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_timestamp(ts: i64) -> String {
    if ts <= 0 {
        return "N/A".into();
    }
    let secs = ts as u64;
    let days_since_epoch = secs / 86400;
    let days_from_1970 = days_since_epoch as i64;
    let mut year = 1970i64;
    let mut remaining = days_from_1970;
    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }
    let months = [
        31,
        if is_leap(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 0u32;
    for (i, &m) in months.iter().enumerate() {
        if remaining < m as i64 {
            month = (i + 1) as u32;
            break;
        }
        remaining -= m as i64;
    }
    format!("{:04}-{:02}-{:02}", year, month, remaining + 1)
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut result = Vec::new();
    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            result.push(String::new());
            continue;
        }

        let mut current_line = String::new();
        for word in paragraph.split_whitespace() {
            if current_line.len() + word.len() + 1 > max_width && !current_line.is_empty() {
                result.push(current_line);
                current_line = String::new();
            }
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
        if !current_line.is_empty() {
            result.push(current_line);
        }
    }
    result
}

struct UsageExample {
    cmd: String,
    desc: String,
}

fn generate_usage_examples(tool: &crate::catalog::tool::Tool) -> Vec<UsageExample> {
    let name = &tool.name;
    let name_lower = name.to_lowercase();
    let mut examples = Vec::new();

    let bin = name.clone();

    match name_lower.as_str() {
        "nmap" => {
            examples.push(UsageExample {
                cmd: format!("{} -sV -sC <target>", bin),
                desc: "Service/version detection + default scripts on a target".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -p- <target>", bin),
                desc: "Scan all 65535 ports".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -sn 192.168.1.0/24", bin),
                desc: "Ping scan to discover live hosts on a subnet".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -A -T4 <target>", bin),
                desc: "Aggressive scan with OS detection, version, scripts, and traceroute".into(),
            });
        }
        "arp-scan" => {
            examples.push(UsageExample {
                cmd: bin.clone(),
                desc: "Scan the local network for ARP-reachable hosts".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} --localnet", bin),
                desc: "Scan the entire local subnet".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -I eth0 192.168.1.0/24", bin),
                desc: "Scan a specific interface and range".into(),
            });
        }
        "sqlmap" => {
            examples.push(UsageExample {
                cmd: format!("{} -u '<URL>?id=1' --dbs", bin),
                desc: "Test a URL parameter for SQL injection and enumerate databases".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -u '<URL>?id=1' -T users --dump", bin),
                desc: "Dump a specific table from the target".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -r request.txt --batch", bin),
                desc: "Automated test from a saved HTTP request file".into(),
            });
        }
        "nikto" => {
            examples.push(UsageExample {
                cmd: format!("{} -h <target>", bin),
                desc: "Scan a web server for vulnerabilities".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -h <target> -p 8080", bin),
                desc: "Scan a specific port".into(),
            });
        }
        "ffuf" => {
            examples.push(UsageExample {
                cmd: format!("{} -u <URL>/FUZZ -w wordlist.txt", bin),
                desc: "Directory/file brute-force fuzzing".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -u <URL> -X POST -d 'user=FUZZ' -w users.txt", bin),
                desc: "Parameter fuzzing with POST data".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -u <URL>/FUZZ -w wordlist.txt -fc 404", bin),
                desc: "Fuzz with filter to hide 404 responses".into(),
            });
        }
        "gobuster" => {
            examples.push(UsageExample {
                cmd: format!("{} dir -u <URL> -w wordlist.txt", bin),
                desc: "Directory and file brute-forcing".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} dns -d example.com -w subdomains.txt", bin),
                desc: "DNS subdomain enumeration".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} vhost -u <URL> -w vhosts.txt", bin),
                desc: "Virtual host discovery".into(),
            });
        }
        "nuclei" => {
            examples.push(UsageExample {
                cmd: format!("{} -u <URL>", bin),
                desc: "Scan a target with all default templates".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -u <URL> -t cves/", bin),
                desc: "Scan specifically for known CVEs".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -l urls.txt -t severe/", bin),
                desc: "Batch scan URLs for severe vulnerabilities".into(),
            });
        }
        "wireshark-cli" | "tshark" => {
            examples.push(UsageExample {
                cmd: format!("{} -i eth0", bin),
                desc: "Capture packets on a network interface".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -r capture.pcap -Y 'http'", bin),
                desc: "Read a capture file and filter for HTTP traffic".into(),
            });
        }
        "hashcat" => {
            examples.push(UsageExample {
                cmd: format!("{} -m 0 hash.txt wordlist.txt", bin),
                desc: "Crack MD5 hashes with a wordlist".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -m 1000 hash.txt wordlist.txt", bin),
                desc: "Crack NTLM hashes".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -a 3 -m 0 hash.txt '?a?a?a?a?a?a'", bin),
                desc: "Brute-force 6-character alphanumeric password".into(),
            });
        }
        "john" => {
            examples.push(UsageExample {
                cmd: format!("{} hash.txt", bin),
                desc: "Crack password hashes with default settings".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} --wordlist=wordlist.txt hash.txt", bin),
                desc: "Crack with a specific wordlist".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} --show hash.txt", bin),
                desc: "Show previously cracked passwords".into(),
            });
        }
        "hydra" => {
            examples.push(UsageExample {
                cmd: format!("{} -l admin -P wordlist.txt <target> ssh", bin),
                desc: "Brute-force SSH login".into(),
            });
            examples.push(UsageExample { cmd: format!("{} -L users.txt -P wordlist.txt <target> http-post-form '/login:user=^USER^&pass=^PASS^'", bin), desc: "Brute-force a web login form".into() });
        }
        "amass" => {
            examples.push(UsageExample {
                cmd: format!("{} enum -passive -d example.com", bin),
                desc: "Passive subdomain enumeration".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} enum -active -d example.com", bin),
                desc: "Active subdomain enumeration (noisier)".into(),
            });
        }
        "subfinder" => {
            examples.push(UsageExample {
                cmd: format!("{} -d example.com", bin),
                desc: "Find subdomains using passive sources".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -d example.com -o subdomains.txt", bin),
                desc: "Save results to a file".into(),
            });
        }
        "masscan" => {
            examples.push(UsageExample {
                cmd: format!("{} 0.0.0.0/0 -p0-65535 --rate=1000", bin),
                desc: "Internet-wide port scan at 1000 packets/sec".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} 192.168.1.0/24 -p80,443", bin),
                desc: "Scan for common web ports on a subnet".into(),
            });
        }
        "metasploit" | "msfconsole" => {
            examples.push(UsageExample {
                cmd: format!("{} -x 'search eternalblue'", bin),
                desc: "Search for exploits on startup".into(),
            });
            examples.push(UsageExample {
                cmd: bin.clone(),
                desc: "Launch the interactive console".into(),
            });
        }
        "burpsuite" | "burp" => {
            examples.push(UsageExample {
                cmd: bin.clone(),
                desc: "Launch Burp Suite GUI proxy".into(),
            });
        }
        "aircrack-ng" => {
            examples.push(UsageExample {
                cmd: format!("{} -w wordlist.txt capture.cap", bin),
                desc: "Crack WPA/WPA2 handshake from a capture file".into(),
            });
        }
        "binwalk" => {
            examples.push(UsageExample {
                cmd: format!("{} firmware.bin", bin),
                desc: "Scan a firmware image for embedded files".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -e firmware.bin", bin),
                desc: "Extract recognized file types from firmware".into(),
            });
        }
        "radare2" | "r2" => {
            examples.push(UsageExample {
                cmd: format!("{} -A binary", bin),
                desc: "Open binary with auto-analysis".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} binary -c 'aaa; pdf'", bin),
                desc: "Analyze and print disassembly of main".into(),
            });
        }
        "ghidra" => {
            examples.push(UsageExample {
                cmd: format!("{} &", bin),
                desc: "Launch the Ghidra GUI (decompiler + disassembler)".into(),
            });
        }
        "strace" => {
            examples.push(UsageExample {
                cmd: format!("{} <command>", bin),
                desc: "Trace system calls of a command".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -p <pid>", bin),
                desc: "Attach to a running process".into(),
            });
        }
        "ltrace" => {
            examples.push(UsageExample {
                cmd: format!("{} <command>", bin),
                desc: "Trace library calls of a command".into(),
            });
        }
        "foremost" => {
            examples.push(UsageExample {
                cmd: format!("{} -i disk.img -o output/", bin),
                desc: "Carve deleted files from a disk image".into(),
            });
        }
        "volatility3" | "vol" => {
            examples.push(UsageExample {
                cmd: format!("{} -f memory.dmp windows.pslist", bin),
                desc: "List processes from a memory dump".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -f memory.dmp windows.netscan", bin),
                desc: "Show network connections from memory".into(),
            });
        }
        "recon-ng" => {
            examples.push(UsageExample {
                cmd: bin.clone(),
                desc: "Launch the interactive OSINT framework console".into(),
            });
        }
        "theharvester" => {
            examples.push(UsageExample {
                cmd: format!("{} -d example.com -b google", bin),
                desc: "Harvest emails and subdomains from Google".into(),
            });
        }
        "sherlock" => {
            examples.push(UsageExample {
                cmd: format!("{} <username>", bin),
                desc: "Search for a username across social media platforms".into(),
            });
        }
        "wpscan" => {
            examples.push(UsageExample {
                cmd: format!("{} --url <URL>", bin),
                desc: "Scan a WordPress site for vulnerabilities".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} --url <URL> --enumerate u", bin),
                desc: "Enumerate WordPress users".into(),
            });
        }
        "feroxbuster" => {
            examples.push(UsageExample {
                cmd: format!("{} -u <URL>", bin),
                desc: "Recursive content discovery".into(),
            });
            examples.push(UsageExample {
                cmd: format!("{} -u <URL> -w wordlist.txt -x pdf,html", bin),
                desc: "Fuzz with custom extensions".into(),
            });
        }
        "httpx" => {
            examples.push(UsageExample {
                cmd: format!("{} -l urls.txt -sc -title", bin),
                desc: "Probe URLs for HTTP info, status code, and title".into(),
            });
        }
        "dirsearch" => {
            examples.push(UsageExample {
                cmd: format!("{} -u <URL>", bin),
                desc: "Brute-force directories and files".into(),
            });
        }
        "whatweb" => {
            examples.push(UsageExample {
                cmd: format!("{} <URL>", bin),
                desc: "Identify technologies used by a website".into(),
            });
        }
        "wifite" => {
            examples.push(UsageExample {
                cmd: bin.clone(),
                desc: "Automated wireless attack tool (WEP/WPA/WPS)".into(),
            });
        }
        "enum4linux" => {
            examples.push(UsageExample {
                cmd: format!("{} <target>", bin),
                desc: "Enumerate Windows/Samba shares and users via SMB".into(),
            });
        }
        "crackmapexec" | "cme" => {
            examples.push(UsageExample {
                cmd: format!("{} smb 192.168.1.0/24 -u user -p pass", bin),
                desc: "Enumerate SMB hosts with credentials".into(),
            });
        }
        "bloodhound" | "bloodhound-python" => {
            examples.push(UsageExample {
                cmd: format!("{} -d domain -u user -p pass -c All", bin),
                desc: "Collect AD data for BloodHound analysis".into(),
            });
        }
        _ => {
            if !tool.executables.is_empty() {
                let exec_name = tool.executables[0].split('/').next_back().unwrap_or(name);
                examples.push(UsageExample {
                    cmd: exec_name.to_string(),
                    desc: "Run with default settings".into(),
                });
                examples.push(UsageExample {
                    cmd: format!("{} --help", exec_name),
                    desc: "Show help and available options".into(),
                });
            } else {
                examples.push(UsageExample {
                    cmd: format!("{} --help", bin),
                    desc: "Show help and available options".into(),
                });
            }
        }
    }

    examples
}
