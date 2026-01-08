use std::fs::{self, Metadata};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::filter::Filter;
use crate::sort::{SortKey, Sorter};

/// Represents a single entry in the directory tree
#[derive(Debug, Clone)]
pub struct TreeEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub symlink_target: Option<PathBuf>,
    pub metadata: Option<Metadata>,
    pub children: Vec<TreeEntry>,
    pub error: Option<String>,
}

impl TreeEntry {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        let symlink_meta = fs::symlink_metadata(&path).ok();
        let is_symlink = symlink_meta.as_ref().map(|m| m.is_symlink()).unwrap_or(false);

        let metadata = fs::metadata(&path).ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);

        let symlink_target = if is_symlink {
            fs::read_link(&path).ok()
        } else {
            None
        };

        Self {
            path,
            name,
            is_dir,
            is_symlink,
            symlink_target,
            metadata,
            children: Vec::new(),
            error: None,
        }
    }

    /// Get file size in bytes
    pub fn size(&self) -> u64 {
        self.metadata.as_ref().map(|m| m.len()).unwrap_or(0)
    }

    /// Get modification time
    pub fn modified(&self) -> Option<SystemTime> {
        self.metadata.as_ref().and_then(|m| m.modified().ok())
    }

    /// Get file permissions as a string (e.g., "drwxr-xr-x")
    pub fn permissions_string(&self) -> String {
        let meta = match &self.metadata {
            Some(m) => m,
            None => return "----------".to_string(),
        };

        let mode = meta.permissions().mode();
        let file_type = if self.is_dir {
            'd'
        } else if self.is_symlink {
            'l'
        } else {
            '-'
        };

        let user = triplet((mode >> 6) & 0o7, mode & 0o4000 != 0, 's');
        let group = triplet((mode >> 3) & 0o7, mode & 0o2000 != 0, 's');
        let other = triplet(mode & 0o7, mode & 0o1000 != 0, 't');

        format!("{}{}{}{}", file_type, user, group, other)
    }

    /// Check if this is an executable file
    pub fn is_executable(&self) -> bool {
        if self.is_dir {
            return false;
        }
        self.metadata
            .as_ref()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }

    /// Get the type indicator character (like ls -F)
    pub fn type_indicator(&self) -> &'static str {
        if self.is_dir {
            "/"
        } else if self.is_symlink {
            "@"
        } else if self.is_executable() {
            "*"
        } else {
            ""
        }
    }
}

fn triplet(mode: u32, special: bool, special_char: char) -> String {
    let r = if mode & 0o4 != 0 { 'r' } else { '-' };
    let w = if mode & 0o2 != 0 { 'w' } else { '-' };
    let x = if mode & 0o1 != 0 {
        if special {
            special_char
        } else {
            'x'
        }
    } else if special {
        special_char.to_ascii_uppercase()
    } else {
        '-'
    };
    format!("{}{}{}", r, w, x)
}

/// Configuration for tree traversal
#[derive(Debug, Clone)]
pub struct TreeConfig {
    pub show_hidden: bool,
    pub dirs_only: bool,
    pub max_depth: Option<usize>,
    pub follow_symlinks: bool,
    pub full_path: bool,
    pub filter: Filter,
    pub sort_key: SortKey,
    pub sort_reverse: bool,
    pub dirs_first: bool,
}

impl Default for TreeConfig {
    fn default() -> Self {
        Self {
            show_hidden: false,
            dirs_only: false,
            max_depth: None,
            follow_symlinks: false,
            full_path: false,
            filter: Filter::default(),
            sort_key: SortKey::Name,
            sort_reverse: false,
            dirs_first: false,
        }
    }
}

/// Statistics collected during tree traversal
#[derive(Debug, Default)]
pub struct TreeStats {
    pub directories: usize,
    pub files: usize,
}

/// Walk a directory and build a tree structure
pub fn walk_directory(
    path: &Path,
    config: &TreeConfig,
    stats: &mut TreeStats,
    current_depth: usize,
) -> TreeEntry {
    let mut entry = TreeEntry::new(path.to_path_buf());

    // Check depth limit
    if let Some(max_depth) = config.max_depth {
        if current_depth >= max_depth {
            return entry;
        }
    }

    if !entry.is_dir {
        return entry;
    }

    // Read directory contents
    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(e) => {
            entry.error = Some(format!("error opening dir: {}", e));
            return entry;
        }
    };

    let mut children: Vec<TreeEntry> = Vec::new();

    for dir_entry in read_dir.flatten() {
        let child_path = dir_entry.path();
        let child_name = child_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        // Skip hidden files unless -a is specified
        if !config.show_hidden && child_name.starts_with('.') {
            continue;
        }

        let child_is_dir = child_path.is_dir();

        // Skip files if dirs_only
        if config.dirs_only && !child_is_dir {
            continue;
        }

        // Apply filters
        if !config.filter.matches(&child_name, child_is_dir) {
            continue;
        }

        // Recursively walk subdirectories
        let child = walk_directory(&child_path, config, stats, current_depth + 1);

        if child.is_dir {
            stats.directories += 1;
        } else {
            stats.files += 1;
        }

        children.push(child);
    }

    // Sort children
    let sorter = Sorter::new(config.sort_key.clone(), config.sort_reverse, config.dirs_first);
    sorter.sort(&mut children);

    entry.children = children;
    entry
}
