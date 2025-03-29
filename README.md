# Lang Checker

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight command-line tool that analyzes your codebase, providing statistics about programming languages used throughout your project.

## Description

Lang Checker recursively scans directories to identify source code files, detect their programming languages, and count lines of code. It generates comprehensive statistics including:

- File counts per language
- Line counts per language  
- Percentage distribution of code across languages

The tool automatically skips `.git` directories and provides a clean, formatted output for easy analysis of your project's composition.

## Installation

### Prerequisites

- Rust and Cargo (1.53.0 or later)

### Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/Goitseone-Themba/lang_checker.git
   cd lang_checker
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The binary will be available at `target/release/lang_checker`

### Optional: Install system-wide

```
cargo install --path .
```

## Usage

Basic usage:
```
lang_checker <directory>
```

Example:
```
lang_checker .
```

This will analyze the current directory and all subdirectories (excluding `.git`).

## Supported Languages

Lang Checker supports a wide range of programming languages, including but not limited to:

- Rust (.rs)
- Python (.py)
- JavaScript (.js)
- TypeScript (.ts)
- Java (.java)
- C/C++ (.c, .cpp, .h, .hpp)
- Go (.go)
- Ruby (.rb)
- HTML (.html, .htm)
- CSS (.css)
- PHP (.php)
- Shell (.sh)
- JSON (.json)
- YAML (.yml, .yaml)
- Markdown (.md)
- And many more!

## Sample Output

Individual file information:
```
File: src/main.rs
  Language: Rust
  Lines: 42

File: src/utils/helper.js
  Language: JavaScript
  Lines: 28
```

Summary statistics:
```
┌────────────┬───────────┬───────────┬──────────┐
│ Language   │ Files     │ Lines     │ Percent  │
├────────────┼───────────┼───────────┼──────────┤
│ Rust       │ 5         │ 283       │ 67.2%    │
│ JavaScript │ 3         │ 78        │ 18.5%    │
│ Python     │ 2         │ 60        │ 14.3%    │
└────────────┴───────────┴───────────┴──────────┘
Total: 10 files, 421 lines of code
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

