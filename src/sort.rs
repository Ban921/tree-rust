use crate::tree::TreeEntry;

/// Sort key options
#[derive(Debug, Clone, Default)]
pub enum SortKey {
    #[default]
    Name,
    Size,
    Time,
    None,
}

impl SortKey {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "name" => SortKey::Name,
            "size" => SortKey::Size,
            "mtime" | "time" => SortKey::Time,
            "none" => SortKey::None,
            _ => SortKey::Name,
        }
    }
}

/// Sorter for tree entries
pub struct Sorter {
    key: SortKey,
    reverse: bool,
    dirs_first: bool,
}

impl Sorter {
    pub fn new(key: SortKey, reverse: bool, dirs_first: bool) -> Self {
        Self {
            key,
            reverse,
            dirs_first,
        }
    }

    pub fn sort(&self, entries: &mut [TreeEntry]) {
        if matches!(self.key, SortKey::None) && !self.dirs_first {
            return;
        }

        entries.sort_by(|a, b| {
            // Dirs first handling
            if self.dirs_first {
                match (a.is_dir, b.is_dir) {
                    (true, false) => return std::cmp::Ordering::Less,
                    (false, true) => return std::cmp::Ordering::Greater,
                    _ => {}
                }
            }

            let ordering = match self.key {
                SortKey::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortKey::Size => a.size().cmp(&b.size()),
                SortKey::Time => {
                    let a_time = a.modified();
                    let b_time = b.modified();
                    match (a_time, b_time) {
                        (Some(at), Some(bt)) => at.cmp(&bt),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                }
                SortKey::None => std::cmp::Ordering::Equal,
            };

            if self.reverse {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }
}
