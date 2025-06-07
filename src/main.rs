use clap::Parser;
use colored::Colorize;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Show hidden files
    #[arg(short = 'a', long, default_value_t = false)]
    all: bool,

    /// Max depth of traversal
    #[arg(short, long, default_value_t = 1)]
    depth: usize,

    /// Search pattern
    #[arg(short, long)]
    search: Option<String>,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "." && s != "..")
        .unwrap_or(false)
}

fn print_entry(entry: &DirEntry, depth: usize) {
    let indent = if depth == 1 {
        String::new()
    } else {
        format!("{}├─ ", "|  ".repeat(depth - 2))
    };
    let file_name = entry.file_name().to_string_lossy();
    if entry.file_type().is_dir() {
        println!("{}{}/", indent, file_name.blue());
    } else {
        println!("{}{}", indent, file_name.green());
    }
}

fn main() {
    let cli = Cli::parse();

    let search = cli.search.as_ref().map(|s| s.to_lowercase());

    // Set max_depth: if level is 0, use usize::MAX; otherwise, use the provided level (or usize::MAX if searching and level==1)
    let max_depth = if (cli.depth == 1 && cli.search.is_some()) || cli.depth == 0 {
        usize::MAX
    } else {
        cli.depth
    };

    // Collect all entrie
    let entries: Vec<_> = WalkDir::new(&cli.path)
        .min_depth(1)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| cli.all || !is_hidden(e))
        .filter_map(Result::ok)
        .collect();

    // Find all directories that are ancestors of matching files
    let mut show_dirs = std::collections::HashSet::new();
    if let Some(ref pattern) = search {
        for entry in &entries {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.contains(pattern) {
                let mut path = entry.path();
                while let Some(parent) = path.parent() {
                    show_dirs.insert(parent.to_path_buf());
                    path = parent;
                }
            }
        }
    }

    for entry in entries {
        let depth = entry.depth();
        let file_name = entry.file_name().to_string_lossy();
        let should_print = if let Some(ref pattern) = search {
            let name = file_name.to_lowercase();
            name.contains(pattern) || show_dirs.contains(entry.path())
        } else {
            true
        };
        if should_print {
            print_entry(&entry, depth);
        }
    }
}
