# nva

A fast, colorful CLI tool for listing directories, inspired by `tree` and designed for developers who live in the terminal.

## Features
- Prints directory trees with indentation and Unicode symbols
- Colors directories blue and files green for easy distinction
- Supports filtering hidden files and directories (like `.git`)
- Adjustable max depth for traversal (`--depth` or `-d`)
- Option to show or hide hidden files/directories with `-a`/`--all`
- Fast and cross-platform (Rust)
- Search for files or directories by name with `-s`/`--search`

## Installation

```
cargo install --path .
```

Or clone and build manually:

```
git clone https://github.com/armanmaurya/nva.git
cd nva
cargo build --release
```

## Usage

```
nva [OPTIONS]
```

### Options

- `-p, --path <PATH>`: Path to the directory to explore (default: current directory)
- `-a, --all`: Show hidden files and directories
- `-d, --depth <DEPTH>`: Max depth of traversal (default: 1, use 0 for unlimited)
- `-s, --search <PATTERN>`: Search for files or directories by name (case-insensitive)

### Example

List the directory tree up to 2 levels deep:
```
nva -d 2
```

Output:
```
.
├─ Cargo.toml
├─ README.md
├─ src/
│  ├─ main.rs
└─ target/
```

Search for all files and directories containing "main":
```
nva --search main
```

## License

MIT