mod error;
mod file_info;
mod language;
mod stats;

use error::{LangCheckerError, Result};
use file_info::FileInfo;
use language::{count_lines, detect_language};
use stats::Stats;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    // Validate the provided directory
    let dir_path = PathBuf::from(&args[1]);
    if !dir_path.exists() {
        return Err(LangCheckerError::PathNotFound(dir_path.display().to_string()));
    }
    if !dir_path.is_dir() {
        return Err(LangCheckerError::InvalidPath(format!(
            "{} is not a directory",
            dir_path.display()
        )));
    }

    // Initialize statistics tracker
    let mut stats = Stats::new();
    
    // Process files recursively
    for entry in WalkDir::new(&dir_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            
            // Skip git directories
            if path.to_string_lossy().contains("/.git/") {
                continue;
            }
            
            // Detect language based on file extension
            let language = detect_language(path);
            
            // Count lines in the file
            let content = match fs::read_to_string(path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading {}: {}", path.display(), e);
                    continue;
                }
            };
            let line_count = count_lines(&content);
            
            // Create FileInfo instance
            let file_info = FileInfo {
                path: path.to_path_buf(),
                language: language.clone(),
                line_count,
            };
            
            // Print file info and update statistics
            println!("{}", file_info);
            stats.add_file(&language, line_count);
        }
    }
    
    // Print summary statistics
    println!("\n{}", stats);

    Ok(())
}
