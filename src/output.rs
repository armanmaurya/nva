use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use colored::Colorize;
use humansize::{format_size, DECIMAL};
use std::io::Write;
use walkdir::DirEntry;

// Print the content of a file with syntax highlighting if supported, otherwise plain text.
pub fn print_file_with_syntax_highlighting(path: &std::path::Path) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    match std::fs::read_to_string(path) {
        Ok(content) => {
            // Highlight if syntect supports the extension, otherwise print plain
            let ps = syntect::parsing::SyntaxSet::load_defaults_newlines();
            if ps.find_syntax_by_extension(ext).is_some() {
                if let Err(e) = highlight_with_syntect(&content, ext) {
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

// Highlight the content of a file using syntect.
pub fn highlight_with_syntect(content: &str, ext: &str) -> Result<(), Box<dyn std::error::Error>> {
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


pub fn output_line<W: Write>(
    writer: &mut W,
    entry: &DirEntry,
    depth: usize,
    use_color: bool,
) -> std::io::Result<()> {
    let indent = if depth == 1 {
        String::new()
    } else {
        format!("{}├─ ", "|  ".repeat(depth - 2))
    };
    let file_name = entry.file_name().to_string_lossy();
    if entry.file_type().is_dir() {
        if use_color {
            writeln!(writer, "{}{}/", indent, file_name.blue())
        } else {
            writeln!(writer, "{}{}/", indent, file_name)
        }
    } else {
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let human_size = format_size(size, DECIMAL);
        if use_color {
            writeln!(
                writer,
                "{}{} ({})",
                indent,
                file_name.green(),
                human_size.yellow()
            )
        } else {
            writeln!(writer, "{}{} ({})", indent, file_name, human_size)
        }
    }
}


// Output the directory tree to a writer, filtering entries based on search criteria.
pub fn output_tree<W: Write>(
    writer: &mut W,
    entries: &[DirEntry],
    show_dirs: &std::collections::HashSet<std::path::PathBuf>,
    search: Option<&str>,
    use_color: bool,
) -> std::io::Result<()> {
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
            output_line(writer, entry, depth, use_color)?;
        }
    }
    Ok(())
}