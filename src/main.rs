use clap::{Parser, ValueEnum};
use walkdir::WalkDir;
mod utils;
use utils::{export_tree_to_file, is_hidden, print_tree, print_with_highlighting};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Show hidden files
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Max depth of traversal
    #[arg(short, long, default_value_t = 1)]
    depth: usize,

    /// Search pattern
    #[arg(short, long)]
    find: Option<String>,

    /// Output file path
    #[arg(short, long)]
    output: Option<String>,

    /// Output in reverse order
    #[arg(long, default_value_t = false)]
    reverse: bool,
}

/// Print the content of a file with syntax highlighting if supported, otherwise plain text.
fn print_file(path: &std::path::Path) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    match std::fs::read_to_string(path) {
        Ok(content) => {
            // Highlight if syntect supports the extension, otherwise print plain
            let ps = syntect::parsing::SyntaxSet::load_defaults_newlines();
            if ps.find_syntax_by_extension(ext).is_some() {
                if let Err(e) = print_with_highlighting(&content, ext) {
                    eprintln!("{}", content); // fallback: plain print
                    eprintln!("[Warning: highlighting failed: {}]", e);
                }
            } else {
                println!("{}", content);
            }
        }
        Err(e) => {
            eprintln!("Failed to read file {}: {}", path.display(), e);
        }
    }
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next(); // skip executable name
    if let Some(arg1) = args.next() {
        let path = std::path::Path::new(&arg1);
        if path.is_file() {
            print_file(path);
            return;
        }
    }
    let cli = Cli::parse();

    let search = cli.find.as_ref().map(|s| s.to_lowercase());

    // Set max_depth: if level is 0, use usize::MAX; otherwise, use the provided level (or usize::MAX if searching and level==1)
    let max_depth = if (cli.depth == 1 && cli.find.is_some()) || cli.depth == 0 {
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

    // if output is specified, write to file and don't print to stdout
    if let Some(output_path) = &cli.output {
        export_tree_to_file(&entries, &show_dirs, search.as_deref(), output_path)
            .expect("Failed to export tree to file");
        println!("Tree exported to {}", output_path);
        return;
    }

    // Use print_tree for terminal output
    print_tree(&entries, &show_dirs, search.as_deref());
}
