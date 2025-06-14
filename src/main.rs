use clap::Parser;
use std::io::Write;
use walkdir::WalkDir;
mod utils;
mod output;
use output::{output_tree};
use output::print_file_with_syntax_highlighting;
use utils::is_hidden;

#[derive(Parser, Debug)]
struct Cli {
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



fn main() {
    let mut args = std::env::args();
    let exe = args.next().unwrap();
    if let Some(arg1) = args.next() {
        let path = std::path::Path::new(&arg1);
        if path.is_file() {
            print_file_with_syntax_highlighting(path);
            return;
        } else if path.is_dir() {
            // Remove the path argument and pass the rest to print_dir
            let remaining_args: Vec<String> = std::iter::once(exe).chain(args).collect();
            print_dir(path, remaining_args);
            return;
        } else {
            eprintln!("{} is not a valid file or directory", arg1);
            std::process::exit(1);
        }
    } else {
        // No argument: pass all args (just the exe) to print_dir, default to current directory
        print_dir(std::path::Path::new("."), std::env::args().collect());
    }
}

fn print_dir(path: &std::path::Path, args: Vec<String>) {
    let cli = Cli::parse_from(args);
    let search = cli.find.as_ref().map(|s| s.to_lowercase());
    let max_depth = if (cli.depth == 1 && cli.find.is_some()) || cli.depth == 0 {
        usize::MAX
    } else {
        cli.depth
    };
    let entries: Vec<_> = WalkDir::new(path)
        .min_depth(1)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| cli.all || !is_hidden(e))
        .filter_map(Result::ok)
        .collect();
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
    if let Some(output_path) = &cli.output {
        let mut file = std::fs::File::create(output_path)
            .expect("Failed to create output file");
        writeln!(file, ".").ok();
        output_tree(&mut file, &entries, &show_dirs, search.as_deref(), false)
            .expect("Failed to export tree to file");
        println!("Tree exported to {}", output_path);
        return;
    }
    // Print to terminal with color
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    output_tree(&mut handle, &entries, &show_dirs, search.as_deref(), true).expect("Failed to print tree");
}
