use glob::Pattern;

/// Filter configuration for file matching
#[derive(Debug, Clone, Default)]
pub struct Filter {
    /// Patterns to include (empty means include all)
    pub include_patterns: Vec<Pattern>,
    /// Patterns to exclude
    pub exclude_patterns: Vec<Pattern>,
    /// Whether pattern matching is case-insensitive
    pub ignore_case: bool,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an include pattern (-P)
    pub fn add_include(&mut self, pattern: &str) -> Result<(), glob::PatternError> {
        let pattern_str = if self.ignore_case {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };
        self.include_patterns.push(Pattern::new(&pattern_str)?);
        Ok(())
    }

    /// Add an exclude pattern (-I)
    pub fn add_exclude(&mut self, pattern: &str) -> Result<(), glob::PatternError> {
        let pattern_str = if self.ignore_case {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };
        self.exclude_patterns.push(Pattern::new(&pattern_str)?);
        Ok(())
    }

    /// Check if a filename matches the filter criteria
    pub fn matches(&self, name: &str, _is_dir: bool) -> bool {
        let match_name = if self.ignore_case {
            name.to_lowercase()
        } else {
            name.to_string()
        };

        // Check exclude patterns first
        for pattern in &self.exclude_patterns {
            if pattern.matches(&match_name) {
                return false;
            }
        }

        // Check include patterns (if any are specified)
        if !self.include_patterns.is_empty() {
            for pattern in &self.include_patterns {
                if pattern.matches(&match_name) {
                    return true;
                }
            }
            return false;
        }

        true
    }
}
