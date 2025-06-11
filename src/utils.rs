use colored::Colorize;
use humansize::{format_size, DECIMAL};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use walkdir::DirEntry;

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
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let human_size = format_size(size, DECIMAL);
        println!("{}{} ({})", indent, file_name.green(), human_size.yellow());
    }
}

pub fn print_tree(
    entries: &[DirEntry],
    show_dirs: &std::collections::HashSet<std::path::PathBuf>,
    search: Option<&str>,
) {
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
            print_entry(entry, depth);
        }
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

pub fn print_with_highlighting(content: &str, ext: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps
        .find_syntax_by_extension(ext)
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(content) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps)?;
        print!("{}", as_24_bit_terminal_escaped(&ranges[..], false));
    }
    Ok(())
}
