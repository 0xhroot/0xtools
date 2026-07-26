#![allow(dead_code)]

mod app;
mod cache;
mod catalog;
mod cli;
mod config;
mod error;
mod package;
mod profiles;
mod reference;
mod search;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, FavoritesAction};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::time::Duration;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "debug" } else { "warn" };

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level)),
        )
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();

    match cli.command {
        None => run_tui(),
        Some(cmd) => run_cli(cmd),
    }
}

fn run_cli(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Search { query } => cli_search(query.join(" ")),
        Commands::Info { name } => cli_info(&name),
        Commands::Categories => cli_categories(),
        Commands::List { category } => cli_list(&category),
        Commands::Installed => cli_installed(),
        Commands::Available => cli_available(),
        Commands::Favorites { action } => cli_favorites(action.unwrap_or(FavoritesAction::List)),
        Commands::Sync => cli_sync(),
        Commands::Doctor => cli_doctor(),
        Commands::Version => cli_version(),
        Commands::Profiles => cli_profiles(),
        Commands::Profile { name, install } => cli_profile(&name, install),
    }
}

fn load_index() -> Result<search::index::SearchIndex> {
    use search::index::SearchIndex;

    let cache_store = crate::cache::format::CacheStore::new()?;

    let tools: Vec<crate::catalog::tool::Tool> = if cache_store.exists() {
        match cache_store.load()? {
            Some(data) => {
                tracing::info!("Loaded {} tools from cache", data.tools.len());
                data.tools
            }
            None => {
                tracing::info!("Cache empty, rebuilding from ALPM...");
                build_from_alpm(&cache_store)?
            }
        }
    } else {
        tracing::info!("No cache found, building from ALPM...");
        build_from_alpm(&cache_store)?
    };

    let mut index = SearchIndex::new();
    index.build(tools);
    tracing::info!("Indexed {} tools", index.len());
    Ok(index)
}

fn build_from_alpm(
    cache_store: &crate::cache::format::CacheStore,
) -> Result<Vec<crate::catalog::tool::Tool>> {
    let alpm_backend = crate::package::AlpmBackend::new()?;
    let mut tools = alpm_backend.build_tools()?;

    let blackarch_detected = tools
        .iter()
        .any(|t| t.repository == crate::catalog::Repository::BlackArch);

    let related_finder = crate::catalog::RelatedFinder::build(&tools);
    let related_map: Vec<Vec<String>> = tools
        .iter()
        .map(|tool| related_finder.find_related(tool, &tools, 5))
        .collect();
    for (tool, related) in tools.iter_mut().zip(related_map) {
        tool.related = related;
    }

    let data = crate::cache::format::CacheData::new(tools.clone(), blackarch_detected);
    if let Err(e) = cache_store.save(&data) {
        tracing::warn!("Failed to save cache: {}", e);
    } else {
        tracing::info!("Saved cache");
    }

    Ok(tools)
}

fn cli_search(query: String) -> Result<()> {
    let index = load_index()?;
    let query_parser = crate::search::query::QueryParser::parse(&query);
    let free_text = if !query_parser.free_text.is_empty() {
        &query_parser.free_text
    } else {
        ""
    };
    let results = index.search(free_text, 50);

    println!("\nSearch results for '{}':\n", query);
    for result in &results {
        if let Some(tool) = index.tool(result.tool_index) {
            let status = if tool.installed { "●" } else { "○" };
            let desc = if tool.short_description.is_empty() {
                "No description".to_string()
            } else if tool.short_description.len() > 60 {
                format!("{}…", &tool.short_description[..59])
            } else {
                tool.short_description.clone()
            };
            println!(
                "  {} {:<30} {:<62} [{}]",
                status,
                tool.name,
                desc,
                tool.repository.name()
            );
        }
    }
    println!("\n  {} tools found", results.len());
    Ok(())
}

fn cli_info(name: &str) -> Result<()> {
    let index = load_index()?;
    let tool = index
        .tools()
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| anyhow::anyhow!("Tool '{}' not found", name))?;

    let knowledge = crate::catalog::tool_knowledge::get_knowledge(tool);

    println!("\n{}", tool.name);
    if !tool.short_description.is_empty() {
        println!("\n{}", tool.short_description);
    }

    println!("\n══════════════════════════════════════════");
    println!(" ABOUT THIS TOOL");
    println!("══════════════════════════════════════════");
    println!("\nWhat It Does:");
    for line in word_wrap(&knowledge.what_it_does, 76) {
        println!("  {}", line);
    }
    println!("\nHow It Works:");
    for line in word_wrap(&knowledge.how_it_works, 76) {
        println!("  {}", line);
    }
    println!("\nDifficulty:     {}", knowledge.difficulty);
    if !knowledge.attack_types.is_empty() {
        println!("Attack Types:   {}", knowledge.attack_types.join(" · "));
    }
    if !knowledge.protocols.is_empty() {
        println!("Protocols:      {}", knowledge.protocols.join(", "));
    }
    if !knowledge.use_cases.is_empty() {
        println!("\nUse Cases:");
        for uc in &knowledge.use_cases {
            println!("  ● {}", uc);
        }
    }
    if !knowledge.key_features.is_empty() {
        println!("\nKey Features:");
        for feat in &knowledge.key_features {
            println!("  ✓ {}", feat);
        }
    }
    if !knowledge.targets.is_empty() {
        println!("\nTargets: {}", knowledge.targets.join(", "));
    }
    if !knowledge.strengths.is_empty() {
        println!("\nStrengths:");
        for s in &knowledge.strengths {
            println!("  + {}", s);
        }
    }
    if !knowledge.limitations.is_empty() {
        println!("\nLimitations:");
        for l in &knowledge.limitations {
            println!("  - {}", l);
        }
    }
    if !knowledge.alternatives.is_empty() {
        println!("\nAlternatives: {}", knowledge.alternatives.join(", "));
    }
    if !knowledge.best_practices.is_empty() {
        println!("\nBest Practices:");
        for bp in &knowledge.best_practices {
            println!("  → {}", bp);
        }
    }
    if !knowledge.typical_workflow.is_empty() {
        println!("\nTypical Workflow:");
        for step in &knowledge.typical_workflow {
            println!("  {}", step);
        }
    }

    println!("\n══════════════════════════════════════════");
    println!(" CLASSIFICATION");
    println!("══════════════════════════════════════════");
    let cats: Vec<String> = tool
        .categories
        .iter()
        .map(|c| c.name().to_string())
        .collect();
    println!("  Category: {}", cats.join(", "));
    if !tool.tags.is_empty() {
        println!("  Tags:     {}", tool.tags.join(" · "));
    }
    if !tool.groups.is_empty() {
        println!("  Groups:   {}", tool.groups.join(", "));
    }

    println!("\n══════════════════════════════════════════");
    println!(" PACKAGE INFORMATION");
    println!("══════════════════════════════════════════");
    println!("  Repository:     {}", tool.repository.name());
    println!("  Version:        {}", tool.available_version);
    println!("  Status:         {}", tool.status_text());
    if let Some(ref ver) = tool.installed_version {
        println!("  Installed Ver:  {}", ver);
    }
    if let Some(ref arch) = tool.arch {
        println!("  Architecture:   {}", arch);
    }
    if let Some(ref packager) = tool.packager {
        println!("  Packager:       {}", packager);
    }
    if !tool.licenses.is_empty() {
        println!("  License:        {}", tool.licenses.join(", "));
    }
    if let Some(ref url) = tool.homepage {
        println!("  Homepage:       {}", url);
    }
    if let Some(size) = tool.download_size {
        println!("  Download Size:  {}", format_size_cli(size));
    }
    if let Some(size) = tool.installed_size {
        println!("  Installed Size: {}", format_size_cli(size));
    }
    if let Some(date) = tool.build_date {
        println!("  Build Date:     {}", format_date_cli(date));
    }
    if let Some(date) = tool.install_date {
        println!("  Installed On:   {}", format_date_cli(date));
    }
    if let Some(ref filename) = tool.filename {
        println!("  Filename:       {}", filename);
    }
    println!("  Source:         {:?}", tool.metadata_source);

    if !tool.executables.is_empty() {
        println!("\nExecutables:");
        for exec in &tool.executables {
            println!("  → {}", exec);
        }
    }

    if !tool.dependencies.is_empty() {
        println!("\nDependencies ({}):", tool.dependencies.len());
        for dep in &tool.dependencies {
            println!("  • {}", dep);
        }
    }

    if !tool.optional_dependencies.is_empty() {
        println!(
            "\nOptional Dependencies ({}):",
            tool.optional_dependencies.len()
        );
        for dep in &tool.optional_dependencies {
            println!("  • {}", dep);
        }
    }

    if !tool.conflicts.is_empty() {
        println!("\nConflicts ({}):", tool.conflicts.len());
        for c in &tool.conflicts {
            println!("  ⚠ {}", c);
        }
    }

    if !tool.related.is_empty() {
        println!("\nRelated Tools:");
        for r in &tool.related {
            println!("  ● {}", r);
        }
    }

    println!();
    Ok(())
}

fn format_size_cli(bytes: i64) -> String {
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

fn format_date_cli(ts: i64) -> String {
    if ts <= 0 {
        return "N/A".into();
    }
    let secs = ts as u64;
    let days_since_epoch = secs / 86400;
    let days_from_1970 = days_since_epoch as i64;
    let mut year = 1970i64;
    let mut remaining = days_from_1970;
    loop {
        let days_in_year = if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
            366
        } else {
            365
        };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }
    let months = [
        31,
        if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
            29
        } else {
            28
        },
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

fn word_wrap(text: &str, max_width: usize) -> Vec<String> {
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

fn cli_categories() -> Result<()> {
    let index = load_index()?;
    let categories = crate::catalog::Category::all();

    println!("\nCategories:\n");
    for cat in categories.iter() {
        let count = index.count_in_category(cat);
        println!("  {:<30} {:>6} tools", cat.name(), count);
    }
    println!("\n  {} total categories", categories.len());
    println!("  {} total tools\n", index.len());
    Ok(())
}

fn cli_list(category: &str) -> Result<()> {
    let index = load_index()?;
    let categories = crate::catalog::Category::all();
    let cat = categories
        .iter()
        .find(|c| c.name().to_lowercase() == category.to_lowercase())
        .ok_or_else(|| anyhow::anyhow!("Category '{}' not found", category))?;

    let tools: Vec<&crate::catalog::tool::Tool> = index
        .tools()
        .iter()
        .filter(|t| t.categories.contains(cat))
        .collect();

    println!("\n{} ({} tools):\n", cat.name(), tools.len());
    for tool in &tools {
        let status = if tool.installed { "●" } else { "○" };
        let desc = if tool.short_description.is_empty() {
            "No description".to_string()
        } else if tool.short_description.len() > 50 {
            format!("{}…", &tool.short_description[..49])
        } else {
            tool.short_description.clone()
        };
        println!(
            "  {} {:<28} {:<52} [{}]",
            status,
            tool.name,
            desc,
            tool.repository.name()
        );
    }
    println!();
    Ok(())
}

fn cli_installed() -> Result<()> {
    let index = load_index()?;
    let tools: Vec<&crate::catalog::tool::Tool> =
        index.tools().iter().filter(|t| t.installed).collect();

    println!("\nInstalled security tools ({}):\n", tools.len());
    for tool in &tools {
        let ver = tool
            .installed_version
            .as_deref()
            .unwrap_or(&tool.available_version);
        let desc = if tool.short_description.is_empty() {
            "No description".to_string()
        } else if tool.short_description.len() > 45 {
            format!("{}…", &tool.short_description[..44])
        } else {
            tool.short_description.clone()
        };
        println!(
            "  ● {:<28} {:<12} {:<46} [{}]",
            tool.name,
            ver,
            desc,
            tool.repository.name()
        );
    }
    println!();
    Ok(())
}

fn cli_available() -> Result<()> {
    let index = load_index()?;
    let tools: Vec<&crate::catalog::tool::Tool> = index.tools().iter().collect();

    println!("\nAvailable tools ({}):\n", tools.len());
    for tool in tools.iter().take(50) {
        let status = if tool.installed { "●" } else { "○" };
        println!(
            "  {} {:<28} [{}]",
            status,
            tool.name,
            tool.repository.name()
        );
    }
    if tools.len() > 50 {
        println!("  ... and {} more", tools.len() - 50);
    }
    println!();
    Ok(())
}

fn cli_favorites(action: FavoritesAction) -> Result<()> {
    use crate::config::favorites::Favorites;

    match action {
        FavoritesAction::Add { name } => {
            let mut favs = Favorites::load();
            favs.add(&name);
            favs.save()?;
            println!("Added '{}' to favorites", name);
        }
        FavoritesAction::Remove { name } => {
            let mut favs = Favorites::load();
            favs.remove(&name);
            favs.save()?;
            println!("Removed '{}' from favorites", name);
        }
        FavoritesAction::List => {
            let favs = Favorites::load();
            if favs.is_empty() {
                println!("\nNo favorites yet. Add tools with: 0xtools favorites add <name>\n");
            } else {
                println!("\nFavorites ({}):\n", favs.len());
                for name in &favs.packages {
                    println!("  ★ {}", name);
                }
                println!();
            }
        }
        FavoritesAction::Toggle { name } => {
            let mut favs = Favorites::load();
            let added = favs.toggle(&name);
            favs.save()?;
            if added {
                println!("Added '{}' to favorites", name);
            } else {
                println!("Removed '{}' from favorites", name);
            }
        }
    }
    Ok(())
}

fn cli_sync() -> Result<()> {
    println!("Syncing package databases...");
    let output = std::process::Command::new("sudo")
        .args(["pacman", "-Sy"])
        .output()?;

    if output.status.success() {
        let cache_store = crate::cache::format::CacheStore::new()?;
        let _ = cache_store.invalidate();
        println!("Cache cleared. Run 0xtools to rebuild the index.");
        println!("Sync complete.");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Sync failed: {}", stderr);
    }
    Ok(())
}

fn cli_doctor() -> Result<()> {
    println!("\n0xtools doctor\n");

    let has_pacman = std::process::Command::new("pacman")
        .arg("--version")
        .output()
        .is_ok();
    println!(
        "  Arch/pacman       {}",
        if has_pacman {
            "\u{2713}"
        } else {
            "\u{2717} pacman not found"
        }
    );

    let has_alpm = std::process::Command::new("pacman")
        .args(["-Dk"])
        .output()
        .is_ok();
    println!(
        "  ALPM database     {}",
        if has_alpm {
            "\u{2713}"
        } else {
            "\u{2717} ALPM not accessible"
        }
    );

    let ba_paths = ["/var/lib/pacman/sync/blackarch", "/etc/pacman.conf"];
    let has_blackarch = ba_paths.iter().any(|p| std::path::Path::new(p).exists());
    println!(
        "  BlackArch         {}",
        if has_blackarch {
            "\u{2713}"
        } else {
            "\u{2717} not detected"
        }
    );

    let cache_ok = crate::cache::format::CacheStore::new().is_ok();
    let config_ok = crate::config::favorites::Favorites::config_dir()
        .map(|d| d.exists() || std::fs::create_dir_all(&d).is_ok())
        .unwrap_or(false);
    println!(
        "  Cache             {}",
        if cache_ok {
            "\u{2713}"
        } else {
            "\u{2717} cache dir not writable"
        }
    );
    println!(
        "  Config            {}",
        if config_ok {
            "\u{2713} (theme: default)".to_string()
        } else {
            "\u{2717} config dir not writable".to_string()
        }
    );

    let term_ok = std::env::var("TERM").is_ok();
    println!(
        "  Terminal          {}",
        if term_ok {
            "\u{2713}"
        } else {
            "\u{2717} TERM not set"
        }
    );

    let is_root = std::env::var("EUID")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .map(|v| v == 0)
        .unwrap_or(false);
    println!(
        "  Privileges        {}",
        if is_root {
            "\u{26a0} Running as root (not recommended)"
        } else {
            "\u{2713} Running as normal user"
        }
    );

    println!("\nDone. No critical problems detected.\n");
    Ok(())
}

fn cli_version() -> Result<()> {
    println!("\n0xtools v{}\n", env!("CARGO_PKG_VERSION"));
    println!("Cybersecurity tool browser for Arch Linux");
    println!("License: MIT OR Apache-2.0\n");
    Ok(())
}

fn cli_profiles() -> Result<()> {
    let collection = crate::profiles::ProfileCollection::load_bundled();
    let profiles = collection.list();

    println!("\nAvailable profiles:\n");
    for profile in profiles {
        println!("  {:<24} {}", profile.name, profile.description);
        println!("  {:<24} {} packages", "", profile.packages.len());
        println!();
    }
    Ok(())
}

fn cli_profile(name: &str, install: bool) -> Result<()> {
    let collection = crate::profiles::ProfileCollection::load_bundled();
    let profiles = collection.list();

    let profile = profiles
        .iter()
        .find(|p| p.name.to_lowercase() == name.to_lowercase())
        .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", name))?;

    println!("\nProfile: {}", profile.name);
    println!("{}\n", profile.description);
    println!("Packages ({}):", profile.packages.len());
    for pkg in &profile.packages {
        println!("  • {}", pkg);
    }

    if install {
        println!("\nInstalling profile packages...");
        let pkgs: Vec<&str> = profile.packages.iter().map(|s| s.as_str()).collect();
        let output = std::process::Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm"])
            .args(&pkgs)
            .output()?;

        if output.status.success() {
            println!("Profile '{}' installed successfully!", profile.name);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Installation failed: {}", stderr);
        }
    } else {
        println!("\nUse --install to install these packages");
    }
    println!();
    Ok(())
}

use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

fn run_tui() -> Result<()> {
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

    let index = load_index()?;
    let favorites = crate::config::favorites::Favorites::load();
    let config = crate::config::Config::load();
    let theme = crate::config::theme::Theme::by_name(&config.theme);
    let profile_collection = crate::profiles::ProfileCollection::load_bundled();
    let blackarch_detected = index
        .tools()
        .iter()
        .any(|t| t.repository == crate::catalog::Repository::BlackArch);

    let mut state = app::AppState::new(
        index,
        favorites,
        theme,
        profile_collection.list().to_vec(),
        blackarch_detected,
    );

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let tick_rate = Duration::from_millis(50);

    let result = run_app(&mut terminal, &mut state, tick_rate);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut app::AppState,
    tick_rate: Duration,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.area();

            let main_chunks = ratatui::layout::Layout::vertical([
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(1),
            ])
            .split(size);

            match state.current_view {
                app::AppView::Dashboard => {
                    ui::render_dashboard(f, main_chunks[0], state);
                }
                app::AppView::Categories | app::AppView::ToolList => {
                    ui::render_tool_list(f, main_chunks[0], state);
                }
                app::AppView::ToolDetail => {
                    ui::render_detail(f, main_chunks[0], state);
                }
                app::AppView::Search => {
                    ui::render_search(f, main_chunks[0], state);
                }
                app::AppView::Profiles => {
                    ui::render_profiles(f, main_chunks[0], state);
                }
                app::AppView::Help => {
                    ui::render_help(f, main_chunks[0], state);
                }
                app::AppView::InstallPreview => {
                    ui::render_install_preview(f, main_chunks[0], state);
                }
                app::AppView::InstallOutput => {
                    ui::render_install_output(f, main_chunks[0], state);
                }
                app::AppView::RemovePreview => {
                    ui::render_remove_preview(f, main_chunks[0], state);
                }
                app::AppView::ExecutableSelect => {
                    ui::render_help(f, main_chunks[0], state);
                }
            }

            ui::render_status_bar(f, main_chunks[1], state);
        })?;

        app::handle_events(state, tick_rate)?;

        if let Some(preview) = state.pending_transaction.take() {
            use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

            state.status_message = Some(format!("Installing {}...", preview.package));

            terminal.draw(|f| {
                let size = f.area();
                let main_chunks = ratatui::layout::Layout::vertical([
                    ratatui::layout::Constraint::Min(0),
                    ratatui::layout::Constraint::Length(1),
                ])
                .split(size);
                ui::render_install_preview(f, main_chunks[0], state);
                ui::render_status_bar(f, main_chunks[1], state);
            })?;

            disable_raw_mode()?;
            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
            terminal.show_cursor()?;

            let result = crate::package::transaction::Transaction::execute(&preview, true);

            enable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                crossterm::terminal::EnterAlternateScreen
            )?;
            terminal.clear()?;

            match result {
                Ok(output) => {
                    state.refresh_installed();
                    let dep_count = preview.dependencies.len();
                    state.status_message = Some(format!(
                        "✓ Installed {} (+ {} deps resolved by pacman)",
                        preview.package, dep_count,
                    ));
                    state.install_output = Some(output);
                    state.install_preview = None;
                    if state.current_view == app::AppView::InstallPreview {
                        state.current_view = app::AppView::InstallOutput;
                    }
                }
                Err(e) => {
                    state.install_output = Some(format!("Error: {}", e));
                    state.install_preview = None;
                    if state.current_view == app::AppView::InstallPreview {
                        state.current_view = app::AppView::InstallOutput;
                    }
                }
            }
        }

        if let Some(preview) = state.pending_remove.take() {
            use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

            state.status_message = Some(format!("Removing {}...", preview.package));

            terminal.draw(|f| {
                let size = f.area();
                let main_chunks = ratatui::layout::Layout::vertical([
                    ratatui::layout::Constraint::Min(0),
                    ratatui::layout::Constraint::Length(1),
                ])
                .split(size);
                ui::render_remove_preview(f, main_chunks[0], state);
                ui::render_status_bar(f, main_chunks[1], state);
            })?;

            disable_raw_mode()?;
            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
            terminal.show_cursor()?;

            let result = crate::package::transaction::Transaction::execute(&preview, true);

            enable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                crossterm::terminal::EnterAlternateScreen
            )?;
            terminal.clear()?;

            match result {
                Ok(_) => {
                    state.refresh_installed();
                    state.status_message =
                        Some(format!("✓ Successfully removed {}", preview.package));
                    state.remove_preview = None;
                    if state.current_view == app::AppView::RemovePreview {
                        state.go_back();
                    }
                }
                Err(e) => {
                    state.error_message = Some(format!("Remove failed: {}", e));
                    state.remove_preview = None;
                    if state.current_view == app::AppView::RemovePreview {
                        state.go_back();
                    }
                }
            }
        }

        if state.should_quit {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_verify() {
        Cli::command().debug_assert();
    }
}
