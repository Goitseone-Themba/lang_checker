use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    let dir_path = PathBuf::from(&args[1]);
    if !dir_path.is_dir() {
        eprintln!("{} is not a directory", dir_path.display());
        std::process::exit(1);
    }

    for entry in fs::read_dir(&dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let ext = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_else(|| "unknown");
            println!("{}: {}", path.display(), ext);
        }
    }

    Ok(())
}
