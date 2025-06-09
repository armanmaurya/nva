use walkdir::DirEntry;
use colored::Colorize;

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "." && s != "..")
        .unwrap_or(false)
}

pub fn print_entry(entry: &DirEntry, depth: usize) {
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

pub fn export_tree_to_file(
    entries: &[DirEntry],
    show_dirs: &std::collections::HashSet<std::path::PathBuf>,
    search: Option<&str>,
    file_path: &str,
) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = std::fs::File::create(file_path)?;
    writeln!(file, ".")?;
    for entry in entries {
        let depth = entry.depth();
        let file_name = entry.file_name().to_string_lossy();
        let should_print = if let Some(pattern) = search {
            let name = file_name.to_lowercase();
            name.contains(pattern) || show_dirs.contains(entry.path())
        } else {
            true
        };
        if should_print {
            let indent = if depth == 1 {
                String::new()
            } else {
                format!("{}├─ ", "|  ".repeat(depth - 2))
            };
            if entry.file_type().is_dir() {
                writeln!(file, "{}{}/", indent, file_name)?;
            } else {
                writeln!(file, "{}{}", indent, file_name)?;
            }
        }
    }
    Ok(())
}
