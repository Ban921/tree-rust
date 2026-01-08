use colored::*;
use serde::Serialize;
use std::io::{self, Write};

use crate::format::{format_size, format_time};
use crate::tree::{TreeEntry, TreeStats};

/// Output format options
#[derive(Debug, Clone, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Toon,
}

/// Configuration for tree printing
#[derive(Debug, Clone)]
pub struct PrintConfig {
    pub colorize: bool,
    pub show_permissions: bool,
    pub show_size: bool,
    pub human_readable: bool,
    pub si_units: bool,
    pub show_date: bool,
    pub time_format: Option<String>,
    pub show_type_indicator: bool,
    pub no_indent: bool,
    pub full_path: bool,
    pub no_report: bool,
    pub output_format: OutputFormat,
}

impl Default for PrintConfig {
    fn default() -> Self {
        Self {
            colorize: true,
            show_permissions: false,
            show_size: false,
            human_readable: false,
            si_units: false,
            show_date: false,
            time_format: None,
            show_type_indicator: false,
            no_indent: false,
            full_path: false,
            no_report: false,
            output_format: OutputFormat::Text,
        }
    }
}

// Tree drawing characters
const BRANCH: &str = "├── ";
const LAST_BRANCH: &str = "└── ";
const VERTICAL: &str = "│   ";
const EMPTY: &str = "    ";

/// Print the tree structure
pub fn print_tree<W: Write>(
    writer: &mut W,
    entry: &TreeEntry,
    config: &PrintConfig,
    stats: &TreeStats,
) -> io::Result<()> {
    match config.output_format {
        OutputFormat::Text => print_tree_text(writer, entry, config, stats),
        OutputFormat::Json => print_tree_json(writer, entry),
        OutputFormat::Toon => print_tree_toon(writer, entry, config),
    }
}

/// Print tree in text format
fn print_tree_text<W: Write>(
    writer: &mut W,
    entry: &TreeEntry,
    config: &PrintConfig,
    stats: &TreeStats,
) -> io::Result<()> {
    // Print root directory
    let root_name = format_entry_name(entry, config, true);
    writeln!(writer, "{}", root_name)?;

    // Print children
    print_children(writer, entry, config, "")?;

    // Print statistics
    if !config.no_report {
        writeln!(writer)?;
        let dir_word = if stats.directories == 1 {
            "directory"
        } else {
            "directories"
        };
        let file_word = if stats.files == 1 { "file" } else { "files" };
        writeln!(
            writer,
            "{} {}, {} {}",
            stats.directories, dir_word, stats.files, file_word
        )?;
    }

    Ok(())
}

fn print_children<W: Write>(
    writer: &mut W,
    entry: &TreeEntry,
    config: &PrintConfig,
    prefix: &str,
) -> io::Result<()> {
    let children = &entry.children;
    let count = children.len();

    for (idx, child) in children.iter().enumerate() {
        let is_last = idx == count - 1;

        // Build the line prefix
        let (branch, child_prefix) = if config.no_indent {
            ("", "".to_string())
        } else if is_last {
            (LAST_BRANCH, format!("{}{}", prefix, EMPTY))
        } else {
            (BRANCH, format!("{}{}", prefix, VERTICAL))
        };

        // Format the entry info
        let mut line = String::new();

        // Add metadata before the name if needed
        if config.show_permissions {
            line.push_str(&child.permissions_string());
            line.push(' ');
        }

        if config.show_size {
            let size_str = if config.human_readable {
                format_size(child.size(), config.si_units)
            } else {
                format!("{:>10}", child.size())
            };
            line.push_str(&size_str);
            line.push(' ');
        }

        if config.show_date {
            if let Some(time) = child.modified() {
                let time_str = format_time(time, config.time_format.as_deref());
                line.push_str(&time_str);
                line.push(' ');
            }
        }

        // Format name with color
        let name = format_entry_name(child, config, false);

        // Print the line
        if config.no_indent {
            writeln!(writer, "{}{}", line, name)?;
        } else {
            writeln!(writer, "{}{}{}{}", prefix, branch, line, name)?;
        }

        // Handle errors
        if let Some(ref error) = child.error {
            let error_prefix = if config.no_indent {
                ""
            } else {
                &child_prefix
            };
            writeln!(writer, "{}{}", error_prefix, error.red())?;
        }

        // Recursively print children
        if !child.children.is_empty() {
            print_children(writer, child, config, &child_prefix)?;
        }
    }

    Ok(())
}

fn format_entry_name(entry: &TreeEntry, config: &PrintConfig, is_root: bool) -> String {
    let name = if config.full_path && !is_root {
        entry.path.to_string_lossy().to_string()
    } else {
        entry.name.clone()
    };

    let mut display_name = if config.colorize {
        if entry.is_dir {
            name.bold().blue().to_string()
        } else if entry.is_symlink {
            name.cyan().to_string()
        } else if entry.is_executable() {
            name.bold().green().to_string()
        } else {
            name
        }
    } else {
        name
    };

    // Add type indicator
    if config.show_type_indicator {
        display_name.push_str(entry.type_indicator());
    }

    // Add symlink target
    if entry.is_symlink {
        if let Some(ref target) = entry.symlink_target {
            let target_str = target.to_string_lossy();
            if config.colorize {
                display_name = format!("{} -> {}", display_name, target_str.cyan());
            } else {
                display_name = format!("{} -> {}", display_name, target_str);
            }
        }
    }

    display_name
}

// JSON/TOML serialization structures
#[derive(Serialize)]
struct TreeNode {
    #[serde(rename = "type")]
    node_type: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<Vec<TreeNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
}

impl From<&TreeEntry> for TreeNode {
    fn from(entry: &TreeEntry) -> Self {
        let node_type = if entry.is_dir {
            "directory"
        } else if entry.is_symlink {
            "link"
        } else {
            "file"
        };

        let contents = if entry.is_dir && !entry.children.is_empty() {
            Some(entry.children.iter().map(TreeNode::from).collect())
        } else {
            None
        };

        let target = entry
            .symlink_target
            .as_ref()
            .map(|p| p.to_string_lossy().to_string());

        TreeNode {
            node_type: node_type.to_string(),
            name: entry.name.clone(),
            contents,
            target,
        }
    }
}

fn print_tree_json<W: Write>(writer: &mut W, entry: &TreeEntry) -> io::Result<()> {
    let tree_node = TreeNode::from(entry);
    let json = serde_json::to_string_pretty(&[tree_node]).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;
    writeln!(writer, "{}", json)?;
    Ok(())
}

/// Print tree in TOON (Token-Oriented Object Notation) format
/// TOON is optimized for LLMs with minimal token usage
fn print_tree_toon<W: Write>(writer: &mut W, entry: &TreeEntry, config: &PrintConfig) -> io::Result<()> {
    writeln!(writer, "# TOON - Tree Output")?;
    print_toon_entry(writer, entry, 0, config)?;
    Ok(())
}

fn print_toon_entry<W: Write>(writer: &mut W, entry: &TreeEntry, depth: usize, config: &PrintConfig) -> io::Result<()> {
    let indent = "  ".repeat(depth);
    let node_type = if entry.is_dir {
        "d"
    } else if entry.is_symlink {
        "l"
    } else {
        "f"
    };

    // Build metadata parts
    let mut parts: Vec<String> = vec![node_type.to_string()];

    if config.show_permissions {
        parts.push(entry.permissions_string());
    }

    if config.show_size {
        let size_str = if config.human_readable {
            format_size(entry.size(), config.si_units)
        } else {
            entry.size().to_string()
        };
        parts.push(size_str);
    }

    if config.show_date {
        if let Some(time) = entry.modified() {
            let time_str = format_time(time, config.time_format.as_deref());
            parts.push(time_str);
        }
    }

    // Add name as last part
    parts.push(entry.name.clone());

    // Output entry: type:perm:size:date:name or type:name
    let line = parts.join(":");
    if let Some(ref target) = entry.symlink_target {
        writeln!(writer, "{}{} -> {}", indent, line, target.display())?;
    } else {
        writeln!(writer, "{}{}", indent, line)?;
    }

    // Output children count if directory has children
    if entry.is_dir && !entry.children.is_empty() {
        for child in &entry.children {
            print_toon_entry(writer, child, depth + 1, config)?;
        }
    }

    Ok(())
}
