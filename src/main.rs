use std::io;
use std::path::PathBuf;

use clap::Parser;
use tree_rust::filter::Filter;
use tree_rust::printer::{print_tree, OutputFormat, PrintConfig};
use tree_rust::sort::SortKey;
use tree_rust::tree::{walk_directory, TreeConfig, TreeStats};

/// A Rust implementation of the Linux tree command
#[derive(Parser, Debug)]
#[command(name = "tree-rust")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to list (default: current directory)
    #[arg(default_value = ".")]
    directory: PathBuf,

    // ===== Listing Options =====
    /// All files are listed (including hidden files)
    #[arg(short = 'a', long = "all")]
    all: bool,

    /// List directories only
    #[arg(short = 'd', long = "dirs-only")]
    dirs_only: bool,

    /// Follow symbolic links like directories
    #[arg(short = 'l', long = "follow")]
    follow_symlinks: bool,

    /// Print the full path prefix for each file
    #[arg(short = 'f', long = "full-path")]
    full_path: bool,

    /// Descend only level directories deep
    #[arg(short = 'L', long = "level")]
    level: Option<usize>,

    /// List only those files that match the pattern
    #[arg(short = 'P', long = "pattern")]
    pattern: Option<Vec<String>>,

    /// Do not list files that match the pattern
    #[arg(short = 'I', long = "ignore")]
    ignore: Option<Vec<String>>,

    /// Ignore case when pattern matching
    #[arg(long = "ignore-case")]
    ignore_case: bool,

    /// Omit the file/directory report at the end
    #[arg(long = "noreport")]
    noreport: bool,

    // ===== File Options =====
    /// Print the protections for each file
    #[arg(short = 'p', long = "perm")]
    permissions: bool,

    /// Print the size in bytes of each file
    #[arg(short = 's', long = "size")]
    size: bool,

    /// Print size in a more human readable way
    #[arg(short = 'h', long = "human")]
    human: bool,

    /// Like -h, but use SI units (powers of 1000)
    #[arg(long = "si")]
    si: bool,

    /// Print the date of last modification
    #[arg(short = 'D', long = "date")]
    date: bool,

    /// Print and format time according to the format
    #[arg(long = "timefmt")]
    timefmt: Option<String>,

    /// Append indicator (like ls -F)
    #[arg(short = 'F', long = "classify")]
    classify: bool,

    // ===== Sorting Options =====
    /// Sort files by last modification time
    #[arg(short = 't', long = "sort-time")]
    sort_time: bool,

    /// Leave files unsorted
    #[arg(short = 'U', long = "unsorted")]
    unsorted: bool,

    /// Reverse the order of the sort
    #[arg(short = 'r', long = "reverse")]
    reverse: bool,

    /// List directories before files
    #[arg(long = "dirsfirst")]
    dirsfirst: bool,

    /// Select sort: name, size, mtime, none
    #[arg(long = "sort")]
    sort: Option<String>,

    // ===== Graphics Options =====
    /// Don't print indentation lines
    #[arg(short = 'i', long = "noindent")]
    noindent: bool,

    /// Turn colorization off always
    #[arg(short = 'n', long = "nocolor")]
    nocolor: bool,

    /// Turn colorization on always
    #[arg(short = 'C', long = "color")]
    color: bool,

    // ===== Output Format Options =====
    /// Print out a JSON representation of the tree
    #[arg(short = 'J', long = "json")]
    json: bool,

    /// Print out a TOON representation of the tree
    #[arg(short = 'T', long = "toon")]
    toon: bool,
}

fn main() {
    let args = Args::parse();

    // Build filter
    let mut filter = Filter::new();
    filter.ignore_case = args.ignore_case;

    if let Some(patterns) = &args.pattern {
        for p in patterns {
            if let Err(e) = filter.add_include(p) {
                eprintln!("Invalid pattern '{}': {}", p, e);
                std::process::exit(1);
            }
        }
    }

    if let Some(ignores) = &args.ignore {
        for p in ignores {
            if let Err(e) = filter.add_exclude(p) {
                eprintln!("Invalid ignore pattern '{}': {}", p, e);
                std::process::exit(1);
            }
        }
    }

    // Determine sort key
    let sort_key = if args.unsorted {
        SortKey::None
    } else if args.sort_time {
        SortKey::Time
    } else if let Some(ref sort_str) = args.sort {
        SortKey::from_str(sort_str)
    } else {
        SortKey::Name
    };

    // Build tree config
    let tree_config = TreeConfig {
        show_hidden: args.all,
        dirs_only: args.dirs_only,
        max_depth: args.level,
        follow_symlinks: args.follow_symlinks,
        full_path: args.full_path,
        filter,
        sort_key,
        sort_reverse: args.reverse,
        dirs_first: args.dirsfirst,
    };

    // Determine colorization
    let colorize = if args.nocolor {
        false
    } else if args.color {
        true
    } else {
        // Auto-detect: colorize if stdout is a tty
        atty::is(atty::Stream::Stdout)
    };

    // Determine output format
    let output_format = if args.json {
        OutputFormat::Json
    } else if args.toon {
        OutputFormat::Toon
    } else {
        OutputFormat::Text
    };

    // Build print config
    let print_config = PrintConfig {
        colorize,
        show_permissions: args.permissions,
        show_size: args.size || args.human || args.si,
        human_readable: args.human || args.si,
        si_units: args.si,
        show_date: args.date,
        time_format: args.timefmt,
        show_type_indicator: args.classify,
        no_indent: args.noindent,
        full_path: args.full_path,
        no_report: args.noreport,
        output_format,
    };

    // Walk the directory
    let mut stats = TreeStats::default();
    let path = args.directory.canonicalize().unwrap_or(args.directory);
    let tree = walk_directory(&path, &tree_config, &mut stats, 0);

    // Print the tree
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if let Err(e) = print_tree(&mut handle, &tree, &print_config, &stats) {
        eprintln!("Error writing output: {}", e);
        std::process::exit(1);
    }
}
