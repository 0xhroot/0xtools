use crate::catalog::Category;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterField {
    Category,
    Repo,
    Tag,
    Installed,
    Favorite,
}

#[derive(Debug, Clone)]
pub struct FilterClause {
    pub field: FilterField,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct StructuredQuery {
    pub filters: Vec<FilterClause>,
    pub free_text: String,
}

impl StructuredQuery {
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty() && self.free_text.is_empty()
    }
}

pub struct QueryParser;

impl QueryParser {
    pub fn parse(input: &str) -> StructuredQuery {
        let mut filters = Vec::new();
        let mut free_parts = Vec::new();

        for part in input.split_whitespace() {
            if let Some((field, value)) = part.split_once(':') {
                let filter_field = match field.to_lowercase().as_str() {
                    "category" | "cat" | "c" => Some(FilterField::Category),
                    "repo" | "repository" => Some(FilterField::Repo),
                    "tag" | "t" => Some(FilterField::Tag),
                    "installed" | "inst" => Some(FilterField::Installed),
                    "favorite" | "fav" | "f" => Some(FilterField::Favorite),
                    _ => None,
                };

                if let Some(ff) = filter_field {
                    filters.push(FilterClause {
                        field: ff,
                        value: value.to_string(),
                    });
                    continue;
                }
            }

            free_parts.push(part);
        }

        StructuredQuery {
            filters,
            free_text: free_parts.join(" "),
        }
    }

    pub fn matches_tool(
        query: &StructuredQuery,
        categories: &[Category],
        tags: &[String],
        installed: bool,
        favorite: bool,
        repo_name: &str,
    ) -> bool {
        for filter in &query.filters {
            match &filter.field {
                FilterField::Category => {
                    let cat_match = categories.iter().any(|c| {
                        c.slug().contains(&filter.value.to_lowercase())
                            || c.name()
                                .to_lowercase()
                                .contains(&filter.value.to_lowercase())
                    });
                    if !cat_match {
                        return false;
                    }
                }
                FilterField::Repo => {
                    if !repo_name
                        .to_lowercase()
                        .contains(&filter.value.to_lowercase())
                    {
                        return false;
                    }
                }
                FilterField::Tag => {
                    let tag_match = tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&filter.value.to_lowercase()));
                    if !tag_match {
                        return false;
                    }
                }
                FilterField::Installed => {
                    let want_installed = filter.value == "true" || filter.value == "yes";
                    if installed != want_installed {
                        return false;
                    }
                }
                FilterField::Favorite => {
                    let want_fav = filter.value == "true" || filter.value == "yes";
                    if favorite != want_fav {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_free_text() {
        let q = QueryParser::parse("nmap scanner");
        assert!(q.filters.is_empty());
        assert_eq!(q.free_text, "nmap scanner");
    }

    #[test]
    fn test_parse_category_filter() {
        let q = QueryParser::parse("category:osint email");
        assert_eq!(q.filters.len(), 1);
        assert_eq!(q.free_text, "email");
    }

    #[test]
    fn test_parse_installed_filter() {
        let q = QueryParser::parse("installed:true");
        assert_eq!(q.filters.len(), 1);
        assert!(matches!(&q.filters[0].field, FilterField::Installed));
    }

    #[test]
    fn test_parse_multiple_filters() {
        let q = QueryParser::parse("category:web installed:true scanner");
        assert_eq!(q.filters.len(), 2);
        assert_eq!(q.free_text, "scanner");
    }

    #[test]
    fn test_matches_tool_no_filters() {
        let q = StructuredQuery {
            filters: vec![],
            free_text: String::new(),
        };
        assert!(QueryParser::matches_tool(
            &q,
            &[],
            &[],
            false,
            false,
            "extra"
        ));
    }
}
