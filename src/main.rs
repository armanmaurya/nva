use clap::Parser;
use walkdir::{WalkDir, DirEntry};
use colored::Colorize;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Show hidden files
    #[arg(short = 'a', long, default_value_t = false)]
    all: bool,

    /// Max depth of traversal
    #[arg(short, long, default_value_t = 1)]
    level: usize,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "." && s != "..")
        .unwrap_or(false)
}

fn print_entry(entry: &DirEntry, depth: usize) {
    let indent = if depth == 1 {
        String::new()
    } else {
        format!("{}├─ ", "│  ".repeat(depth - 1))
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

    println!("."); // Print the root

    for entry in WalkDir::new(&cli.path)
        .min_depth(1)
        .max_depth(cli.level)
        .into_iter()
        .filter_entry(|e| cli.all || !is_hidden(e)) // Only filter hidden directories if all is false
        .filter_map(Result::ok)
    {
        let depth = entry.depth();
        // if depth > cli.level {
        //     continue; // Skip entries deeper than the specified level
        // }
        print_entry(&entry, depth);
    }
}

