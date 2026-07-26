use crate::catalog::tool::Tool;
use crate::catalog::Category;
use crate::config::favorites::Favorites;
use crate::config::theme::Theme;
use crate::profiles::Profile;
use crate::search::index::SearchIndex;
use crate::search::query::QueryParser;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Dashboard,
    Categories,
    ToolList,
    ToolDetail,
    Search,
    Profiles,
    Help,
    InstallPreview,
    InstallOutput,
    RemovePreview,
    ExecutableSelect,
}

pub struct AppState {
    pub current_view: AppView,
    pub previous_view: Option<AppView>,
    pub search_index: SearchIndex,
    pub favorites: Favorites,
    pub theme: Theme,

    pub category_list_state: ListState,
    pub tool_list_state: ListState,
    pub detail_scroll: u16,
    pub help_scroll: u16,

    pub search_input: String,
    pub search_cursor: usize,
    pub search_results: Vec<usize>,
    pub search_result_selected: usize,

    pub selected_tool: Option<usize>,
    pub selected_category: Option<Category>,
    pub visible_tools: Vec<usize>,

    pub profiles: Vec<Profile>,
    pub selected_profile: Option<usize>,

    pub status_message: Option<String>,
    pub error_message: Option<String>,
    pub blackarch_detected: bool,
    pub tool_count: usize,
    pub installed_count: usize,
    pub blackarch_count: usize,

    pub executable_list: Vec<String>,
    pub executable_selected: usize,

    pub install_preview: Option<crate::package::transaction::TransactionPreview>,
    pub install_output: Option<String>,
    pub install_output_scroll: u16,
    pub remove_preview: Option<crate::package::transaction::TransactionPreview>,

    pub pending_transaction: Option<crate::package::transaction::TransactionPreview>,
    pub pending_remove: Option<crate::package::transaction::TransactionPreview>,

    pub visible_category_indices: Vec<usize>,

    pub should_quit: bool,
    pub last_tick: Instant,
}

#[derive(Clone)]
pub struct ListState {
    pub offset: usize,
    pub selected: usize,
    pub len: usize,
}

impl ListState {
    pub fn new() -> Self {
        Self {
            offset: 0,
            selected: 0,
            len: 0,
        }
    }

    pub fn next(&mut self) {
        if self.len == 0 {
            return;
        }
        self.selected = (self.selected + 1) % self.len;
    }

    pub fn previous(&mut self) {
        if self.len == 0 {
            return;
        }
        if self.selected == 0 {
            self.selected = self.len - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn select(&mut self, idx: usize) {
        if idx < self.len {
            self.selected = idx;
        }
    }

    pub fn scroll_down(&mut self, viewport_height: usize) {
        if self.selected >= self.offset + viewport_height {
            self.offset = self.selected - viewport_height + 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.selected < self.offset {
            self.offset = self.selected;
        }
    }

    pub fn scroll_to_visible(&mut self, viewport_height: usize) {
        self.scroll_down(viewport_height);
        self.scroll_up();
    }

    pub fn adjust_offset(&mut self, viewport_height: usize) {
        if self.len == 0 {
            self.offset = 0;
            return;
        }
        if self.selected >= self.len {
            self.selected = self.len - 1;
        }
        if self.selected < self.offset {
            self.offset = self.selected;
        }
        if self.selected >= self.offset + viewport_height {
            self.offset = self.selected.saturating_sub(viewport_height - 1);
        }
    }
}

impl AppState {
    pub fn new(
        search_index: SearchIndex,
        favorites: Favorites,
        theme: Theme,
        profiles: Vec<Profile>,
        blackarch_detected: bool,
    ) -> Self {
        let tool_count = search_index.len();
        let installed_count = search_index.installed_count();
        let blackarch_count = search_index
            .tools()
            .iter()
            .filter(|t| t.repository == crate::catalog::Repository::BlackArch)
            .count();

        let mut state = Self {
            current_view: AppView::Dashboard,
            previous_view: None,
            search_index,
            favorites,
            theme,
            category_list_state: ListState::new(),
            tool_list_state: ListState::new(),
            detail_scroll: 0,
            help_scroll: 0,
            search_input: String::new(),
            search_cursor: 0,
            search_results: Vec::new(),
            search_result_selected: 0,
            selected_tool: None,
            selected_category: None,
            visible_tools: Vec::new(),
            profiles,
            selected_profile: None,
            status_message: None,
            error_message: None,
            blackarch_detected,
            tool_count,
            installed_count,
            blackarch_count,
            executable_list: Vec::new(),
            executable_selected: 0,
            install_preview: None,
            install_output: None,
            install_output_scroll: 0,
            remove_preview: None,
            pending_transaction: None,
            pending_remove: None,
            visible_category_indices: Vec::new(),
            should_quit: false,
            last_tick: Instant::now(),
        };

        state.category_list_state.len = Category::all().len();
        state.rebuild_visible_tools();
        state.rebuild_visible_categories();

        state
    }

    pub fn rebuild_visible_tools(&mut self) {
        self.visible_tools = match self.selected_category {
            Some(cat) => self
                .search_index
                .tools()
                .iter()
                .enumerate()
                .filter(|(_, t)| t.categories.contains(&cat))
                .map(|(i, _)| i)
                .collect(),
            None => (0..self.search_index.len()).collect(),
        };
        self.tool_list_state.len = self.visible_tools.len();
        self.tool_list_state.selected = 0;
        self.tool_list_state.offset = 0;
    }

    pub fn rebuild_visible_categories(&mut self) {
        self.visible_category_indices = Category::all()
            .iter()
            .enumerate()
            .filter(|(_, cat)| self.search_index.count_in_category(cat) > 0)
            .map(|(i, _)| i)
            .collect();
        self.category_list_state.len = self.visible_category_indices.len();
        if self.category_list_state.selected >= self.visible_category_indices.len() {
            self.category_list_state.selected = 0;
        }
    }

    pub fn selected_visible_category(&self) -> Option<Category> {
        let all = Category::all();
        self.visible_category_indices
            .get(self.category_list_state.selected)
            .and_then(|&idx| all.get(idx))
            .copied()
    }

    pub fn select_category(&mut self, idx: usize) {
        let cats = Category::all();
        if idx < cats.len() {
            self.selected_category = Some(cats[idx]);
            self.rebuild_visible_tools();
        }
    }

    pub fn current_tool(&self) -> Option<&Tool> {
        self.selected_tool
            .and_then(|idx| self.search_index.tool(idx))
    }

    pub fn toggle_favorite_current(&mut self) {
        if let Some(tool) = self.current_tool() {
            let name = tool.name.clone();
            self.favorites.toggle(&name);
            let _ = self.favorites.save();
        }
    }

    pub fn navigate_to(&mut self, view: AppView) {
        self.previous_view = Some(self.current_view);
        self.current_view = view;
    }

    pub fn go_back(&mut self) {
        self.install_preview = None;
        self.install_output = None;
        self.install_output_scroll = 0;
        self.remove_preview = None;
        self.error_message = None;
        if let Some(prev) = self.previous_view.take() {
            self.current_view = prev;
        } else {
            self.current_view = AppView::Dashboard;
        }
    }

    pub fn refresh_installed(&mut self) {
        if let Ok(backend) = crate::package::AlpmBackend::new() {
            let installed_names: std::collections::HashSet<String> = match backend.all_packages() {
                Ok(pkgs) => pkgs
                    .iter()
                    .filter(|p| p.installed)
                    .map(|p| p.name.clone())
                    .collect(),
                Err(_) => return,
            };
            for tool in self.search_index.tools_mut().iter_mut() {
                let was_installed = tool.installed;
                tool.installed = installed_names.contains(&tool.name);
                if tool.installed && !was_installed {
                    tool.installed_version = Some(tool.available_version.clone());
                } else if !tool.installed {
                    tool.installed_version = None;
                }
            }
            self.installed_count = self.search_index.installed_count();
        }
    }

    pub fn open_selected_tool(&mut self) {
        let tools = match self.current_view {
            AppView::Search => &self.search_results,
            _ => &self.visible_tools,
        };
        let selected = match self.current_view {
            AppView::Search => self.search_result_selected,
            _ => self.tool_list_state.selected,
        };

        if selected < tools.len() {
            let tool_idx = tools[selected];
            self.selected_tool = Some(tool_idx);
            self.detail_scroll = 0;
            self.navigate_to(AppView::ToolDetail);
        }
    }

    pub fn search(&mut self) {
        let query = self.search_input.clone();
        let parsed = QueryParser::parse(&query);

        if parsed.free_text.is_empty() && parsed.filters.is_empty() {
            self.search_results = Vec::new();
        } else {
            let free_text = if !parsed.free_text.is_empty() {
                &parsed.free_text
            } else {
                ""
            };

            let mut results = self.search_index.search(free_text, 200);

            if !parsed.filters.is_empty() {
                results.retain(|r| {
                    if let Some(tool) = self.search_index.tool(r.tool_index) {
                        let fav_set = self.favorites.as_set();
                        let is_fav = fav_set.contains(&tool.name);
                        QueryParser::matches_tool(
                            &parsed,
                            &tool.categories,
                            &tool.tags,
                            tool.installed,
                            is_fav,
                            &tool.repository.name(),
                        )
                    } else {
                        false
                    }
                });
            }

            self.search_results = results.into_iter().map(|r| r.tool_index).collect();
        }

        self.search_result_selected = 0;
        self.tool_list_state.len = self.search_results.len();
        self.tool_list_state.selected = 0;
        self.tool_list_state.offset = 0;
    }

    pub fn start_install(&mut self) {
        if let Some(tool) = self.current_tool() {
            if !tool.installed {
                let pkg = tool.name.clone();
                match crate::package::transaction::Transaction::preview_install(&pkg) {
                    Ok(preview) => {
                        self.install_preview = Some(preview);
                        self.navigate_to(AppView::InstallPreview);
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to query package: {}", e));
                    }
                }
            }
        }
    }

    pub fn start_remove(&mut self) {
        if let Some(tool) = self.current_tool() {
            if tool.installed {
                let pkg = tool.name.clone();
                match crate::package::transaction::Transaction::preview_remove(&pkg) {
                    Ok(preview) => {
                        self.remove_preview = Some(preview);
                        self.navigate_to(AppView::RemovePreview);
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to prepare removal: {}", e));
                    }
                }
            }
        }
    }
}

pub fn handle_events(state: &mut AppState, tick_rate: Duration) -> anyhow::Result<()> {
    if event::poll(tick_rate)? {
        if let CrosstermEvent::Key(key) = event::read()? {
            match state.current_view {
                AppView::Help => handle_help_input(state, key),
                AppView::Search => handle_search_input(state, key),
                AppView::ToolDetail => handle_detail_input(state, key),
                AppView::InstallPreview => handle_install_preview_input(state, key),
                AppView::InstallOutput => handle_install_output_input(state, key),
                AppView::RemovePreview => handle_remove_preview_input(state, key),
                AppView::ExecutableSelect => handle_executable_input(state, key),
                _ => handle_main_input(state, key),
            }
        }
    }
    Ok(())
}

fn handle_main_input(state: &mut AppState, key: KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            state.should_quit = true;
        }
        (KeyCode::Char('?'), _) => {
            state.navigate_to(AppView::Help);
        }
        (KeyCode::Char('/'), _) | (KeyCode::Char('s'), _) => {
            state.navigate_to(AppView::Search);
            state.search_input.clear();
            state.search_cursor = 0;
        }
        (KeyCode::Char('j'), _) | (KeyCode::Down, _) => match state.current_view {
            AppView::Dashboard => {
                state.category_list_state.next();
                if let Some(cat) = state.selected_visible_category() {
                    state.selected_category = Some(cat);
                    state.rebuild_visible_tools();
                }
            }
            AppView::Categories | AppView::ToolList => {
                state.tool_list_state.next();
            }
            AppView::Profiles => {
                if let Some(sel) = state.selected_profile.as_mut() {
                    if *sel + 1 < state.profiles.len() {
                        *sel += 1;
                    }
                } else if !state.profiles.is_empty() {
                    state.selected_profile = Some(0);
                }
            }
            _ => {}
        },
        (KeyCode::Char('k'), _) | (KeyCode::Up, _) => match state.current_view {
            AppView::Dashboard => {
                state.category_list_state.previous();
                if let Some(cat) = state.selected_visible_category() {
                    state.selected_category = Some(cat);
                    state.rebuild_visible_tools();
                }
            }
            AppView::Categories | AppView::ToolList => {
                state.tool_list_state.previous();
            }
            AppView::Profiles => {
                if let Some(sel) = &mut state.selected_profile {
                    if *sel > 0 {
                        *sel -= 1;
                    }
                }
            }
            _ => {}
        },
        (KeyCode::Char('l'), _) | (KeyCode::Right, _) | (KeyCode::Enter, _) => {
            match state.current_view {
                AppView::Dashboard => {
                    if let Some(cat) = state.selected_visible_category() {
                        state.selected_category = Some(cat);
                        state.rebuild_visible_tools();
                        state.navigate_to(AppView::ToolList);
                    }
                }
                AppView::ToolList => {
                    state.open_selected_tool();
                }
                _ => {}
            }
        }
        (KeyCode::Char('h'), _) | (KeyCode::Left, _) | (KeyCode::Esc, _) => {
            state.go_back();
        }
        (KeyCode::Char('f'), _) => {
            state.toggle_favorite_current();
        }
        (KeyCode::Char('i'), _) => {
            state.start_install();
        }
        (KeyCode::Char('u'), _) => {
            state.start_remove();
        }
        (KeyCode::Char('r'), _) => {
            if let Some(tool) = state.current_tool() {
                if tool.installed && !tool.executables.is_empty() {
                    state.executable_list = tool.executables.clone();
                    state.executable_selected = 0;
                    state.navigate_to(AppView::ExecutableSelect);
                }
            }
        }
        (KeyCode::Tab, _) => match state.current_view {
            AppView::Dashboard => {
                state.navigate_to(AppView::ToolList);
            }
            AppView::ToolList => {
                state.navigate_to(AppView::Dashboard);
            }
            _ => {}
        },
        (KeyCode::Char('p'), _) => {
            if !state.profiles.is_empty() {
                state.selected_profile = Some(0);
                state.navigate_to(AppView::Profiles);
            }
        }
        (KeyCode::Char('n'), _)
            if state.category_list_state.selected + 1 < state.visible_category_indices.len() =>
        {
            state.category_list_state.selected += 1;
        }
        _ => {}
    }
}

fn handle_search_input(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.go_back();
        }
        KeyCode::Enter => {
            if !state.search_results.is_empty() {
                state.open_selected_tool();
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if state.search_result_selected + 1 < state.search_results.len() {
                state.search_result_selected += 1;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if state.search_result_selected > 0 {
                state.search_result_selected -= 1;
            }
        }
        KeyCode::Backspace => {
            if state.search_cursor > 0 {
                state.search_cursor -= 1;
                state.search_input.remove(state.search_cursor);
                state.search();
            }
        }
        KeyCode::Delete => {
            if state.search_cursor < state.search_input.len() {
                state.search_input.remove(state.search_cursor);
                state.search();
            }
        }
        KeyCode::Left => {
            if state.search_cursor > 0 {
                state.search_cursor -= 1;
            }
        }
        KeyCode::Right => {
            if state.search_cursor < state.search_input.len() {
                state.search_cursor += 1;
            }
        }
        KeyCode::Home => {
            state.search_cursor = 0;
        }
        KeyCode::End => {
            state.search_cursor = state.search_input.len();
        }
        KeyCode::Char(c) => {
            state.search_input.insert(state.search_cursor, c);
            state.search_cursor += 1;
            state.search();
        }
        _ => {}
    }
}

fn handle_detail_input(state: &mut AppState, key: KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, _) | (KeyCode::Char('h'), _) | (KeyCode::Left, _) => {
            state.go_back();
        }
        (KeyCode::Down, _) | (KeyCode::Char('j'), _) => {
            state.detail_scroll = state.detail_scroll.saturating_add(1);
        }
        (KeyCode::Up, _) | (KeyCode::Char('k'), _) => {
            state.detail_scroll = state.detail_scroll.saturating_sub(1);
        }
        (KeyCode::Char('f'), _) => {
            state.toggle_favorite_current();
        }
        (KeyCode::Char('i'), _) => {
            state.start_install();
        }
        (KeyCode::Char('u'), _) => {
            state.start_remove();
        }
        (KeyCode::Char('r'), _) => {
            if let Some(tool) = state.current_tool() {
                if tool.installed && !tool.executables.is_empty() {
                    state.executable_list = tool.executables.clone();
                    state.executable_selected = 0;
                    state.navigate_to(AppView::ExecutableSelect);
                }
            }
        }
        _ => {}
    }
}

fn handle_help_input(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.go_back();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.help_scroll = state.help_scroll.saturating_add(1);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.help_scroll = state.help_scroll.saturating_sub(1);
        }
        _ => {}
    }
}

fn handle_install_preview_input(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.go_back();
        }
        KeyCode::Enter => {
            if let Some(preview) = state.install_preview.take() {
                state.pending_transaction = Some(preview);
            }
        }
        _ => {}
    }
}

fn handle_install_output_input(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
            state.install_output = None;
            state.install_output_scroll = 0;
            state.go_back();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.install_output_scroll = state.install_output_scroll.saturating_add(1);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.install_output_scroll = state.install_output_scroll.saturating_sub(1);
        }
        KeyCode::PageDown | KeyCode::Char(' ') => {
            state.install_output_scroll = state.install_output_scroll.saturating_add(20);
        }
        KeyCode::PageUp => {
            state.install_output_scroll = state.install_output_scroll.saturating_sub(20);
        }
        _ => {}
    }
}

fn handle_remove_preview_input(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.go_back();
        }
        KeyCode::Enter => {
            if let Some(preview) = state.remove_preview.take() {
                state.pending_remove = Some(preview);
            }
        }
        _ => {}
    }
}

fn handle_executable_input(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.go_back();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if state.executable_selected + 1 < state.executable_list.len() {
                state.executable_selected += 1;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if state.executable_selected > 0 {
                state.executable_selected -= 1;
            }
        }
        KeyCode::Enter => {
            if let Some(exec) = state.executable_list.get(state.executable_selected) {
                state.status_message = Some(format!("Would launch: {}", exec));
                state.go_back();
            }
        }
        _ => {}
    }
}
