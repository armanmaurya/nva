# nva

A fast, colorful CLI tool for listing directories, inspired by `tree` and designed for developers who live in the terminal.

## Features
- Prints directory trees with indentation and Unicode symbols
- Colors directories blue and files green for easy distinction
- Supports filtering hidden files and directories (like `.git`)
- Adjustable max depth for traversal
- Option to show or hide hidden files/directories with `-a`/`--all`
- Fast and cross-platform (Rust)

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
- `-l, --level <LEVEL>`: Max depth of traversal (default: 1)

### Example

```
nva -l 2
```

Output:
```
.
├── Cargo.toml
├── README.md
├── src/
│  ├─ main.rs
└── target/
```

## License

MIT