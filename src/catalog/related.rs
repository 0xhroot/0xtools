use crate::catalog::tool::Tool;
use hashbrown::HashMap;

pub struct RelatedFinder {
    by_category: HashMap<String, Vec<usize>>,
    by_tags: HashMap<String, Vec<usize>>,
}

impl RelatedFinder {
    pub fn build(tools: &[Tool]) -> Self {
        let mut by_category: HashMap<String, Vec<usize>> = HashMap::new();
        let mut by_tags: HashMap<String, Vec<usize>> = HashMap::new();

        for (idx, tool) in tools.iter().enumerate() {
            for cat in &tool.categories {
                by_category
                    .entry(cat.slug().to_string())
                    .or_default()
                    .push(idx);
            }
            for tag in &tool.tags {
                by_tags.entry(tag.clone()).or_default().push(idx);
            }
        }

        Self {
            by_category,
            by_tags,
        }
    }

    pub fn find_related(&self, tool: &Tool, all_tools: &[Tool], max: usize) -> Vec<String> {
        let mut scores: HashMap<usize, u32> = HashMap::new();

        for cat in &tool.categories {
            if let Some(indices) = self.by_category.get(cat.slug()) {
                for &idx in indices {
                    if all_tools[idx].name != tool.name {
                        *scores.entry(idx).or_insert(0) += 10;
                    }
                }
            }
        }

        for tag in &tool.tags {
            if let Some(indices) = self.by_tags.get(tag.as_str()) {
                for &idx in indices {
                    if all_tools[idx].name != tool.name {
                        *scores.entry(idx).or_insert(0) += 5;
                    }
                }
            }
        }

        let mut scored: Vec<(usize, u32)> = scores.into_iter().collect();
        scored.sort_by_key(|b| std::cmp::Reverse(b.1));

        scored
            .into_iter()
            .take(max)
            .map(|(idx, _)| all_tools[idx].name.clone())
            .collect()
    }
}
