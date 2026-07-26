#[derive(Debug, Clone)]
pub struct SearchResult {
    pub tool_index: usize,
    pub score: u32,
    pub name_matched: bool,
}

impl SearchResult {
    pub fn rank_label(&self) -> &str {
        if self.name_matched {
            "exact"
        } else {
            "fuzzy"
        }
    }
}
