use crate::catalog::tool::Tool;
use crate::catalog::Category;
use crate::search::ranking::SearchResult;
use hashbrown::HashMap;
use nucleo::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo::{Matcher, Utf32Str};

pub struct SearchIndex {
    tools: Vec<Tool>,
    name_to_idx: HashMap<String, usize>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
            name_to_idx: HashMap::new(),
        }
    }

    pub fn build(&mut self, tools: Vec<Tool>) {
        self.name_to_idx.clear();

        for (idx, tool) in tools.iter().enumerate() {
            self.name_to_idx.insert(tool.name.clone(), idx);
        }

        self.tools = tools;
    }

    pub fn search(&self, query: &str, max_results: usize) -> Vec<SearchResult> {
        if query.is_empty() {
            return self
                .tools
                .iter()
                .enumerate()
                .map(|(idx, _tool)| SearchResult {
                    tool_index: idx,
                    score: 0,
                    name_matched: false,
                })
                .take(max_results)
                .collect();
        }

        let pattern = Pattern::new(
            query,
            CaseMatching::Smart,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );

        let mut scored_results: Vec<SearchResult> = Vec::new();
        let mut matcher = Matcher::default();

        for (idx, tool) in self.tools.iter().enumerate() {
            let mut score: u32 = 0;

            let name_str = &tool.name;
            let mut name_buf = Vec::new();
            let name_utf32 = Utf32Str::new(name_str, &mut name_buf);
            if let Some(s) = pattern.score(name_utf32, &mut matcher) {
                score += s * 100;
            }

            let desc_str = &tool.short_description;
            let mut desc_buf = Vec::new();
            let desc_utf32 = Utf32Str::new(desc_str, &mut desc_buf);
            if let Some(s) = pattern.score(desc_utf32, &mut matcher) {
                score += s * 10;
            }

            for cat in &tool.categories {
                let cat_str = cat.name();
                let mut cat_buf = Vec::new();
                let cat_utf32 = Utf32Str::new(cat_str, &mut cat_buf);
                if let Some(s) = pattern.score(cat_utf32, &mut matcher) {
                    score += s * 5;
                }
            }

            for tag in &tool.tags {
                let mut tag_buf = Vec::new();
                let tag_utf32 = Utf32Str::new(tag, &mut tag_buf);
                if let Some(s) = pattern.score(tag_utf32, &mut matcher) {
                    score += s * 3;
                }
            }

            let detail_str = &tool.detailed_description;
            let mut detail_buf = Vec::new();
            let detail_utf32 = Utf32Str::new(detail_str, &mut detail_buf);
            if let Some(s) = pattern.score(detail_utf32, &mut matcher) {
                score += s;
            }

            if score > 0 {
                let name_matched = tool.name.to_lowercase().contains(&query.to_lowercase())
                    || pattern.score(name_utf32, &mut matcher).is_some();

                scored_results.push(SearchResult {
                    tool_index: idx,
                    score,
                    name_matched,
                });
            }
        }

        scored_results.sort_by(|a, b| {
            b.score.cmp(&a.score).then_with(|| {
                let a_name = &self.tools[a.tool_index].name;
                let b_name = &self.tools[b.tool_index].name;
                a_name.cmp(b_name)
            })
        });

        scored_results.truncate(max_results);
        scored_results
    }

    pub fn tool(&self, idx: usize) -> Option<&Tool> {
        self.tools.get(idx)
    }

    pub fn tool_by_name(&self, name: &str) -> Option<&Tool> {
        self.name_to_idx
            .get(name)
            .and_then(|&idx| self.tools.get(idx))
    }

    pub fn tool_idx_by_name(&self, name: &str) -> Option<usize> {
        self.name_to_idx.get(name).copied()
    }

    pub fn tools(&self) -> &[Tool] {
        &self.tools
    }

    pub fn tools_mut(&mut self) -> &mut [Tool] {
        &mut self.tools
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    pub fn installed_count(&self) -> usize {
        self.tools.iter().filter(|t| t.installed).count()
    }

    pub fn tools_in_category(&self, category: &Category) -> Vec<&Tool> {
        self.tools
            .iter()
            .filter(|t| t.categories.contains(category))
            .collect()
    }

    pub fn installed_tools(&self) -> Vec<&Tool> {
        self.tools.iter().filter(|t| t.installed).collect()
    }

    pub fn count_in_category(&self, category: &Category) -> usize {
        self.tools
            .iter()
            .filter(|t| t.categories.contains(category))
            .count()
    }
}
