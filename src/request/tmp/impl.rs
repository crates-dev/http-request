use super::r#type::Tmp;
use std::collections::HashSet;

impl Default for Tmp {
    fn default() -> Self {
        Self {
            visit_url: HashSet::new(),
        }
    }
}
