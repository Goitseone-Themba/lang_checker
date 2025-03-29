use std::path::Path;

pub fn detect_language(file_path: &Path) -> String {
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "rs" => "Rust".to_string(),
            "py" => "Python".to_string(),
            "js" => "JavaScript".to_string(),
            "ts" => "TypeScript".to_string(),
            "java" => "Java".to_string(),
            "c" => "C".to_string(),
            "cpp" | "cc" | "cxx" => "C++".to_string(),
            "h" | "hpp" | "hxx" => "C/C++ Header".to_string(),
            "go" => "Go".to_string(),
            "rb" => "Ruby".to_string(),
            "php" => "PHP".to_string(),
            "html" | "htm" => "HTML".to_string(),
            "css" => "CSS".to_string(),
            "md" | "markdown" => "Markdown".to_string(),
            "json" => "JSON".to_string(),
            "yml" | "yaml" => "YAML".to_string(),
            "toml" => "TOML".to_string(),
            "sh" | "bash" => "Shell".to_string(),
            _ => format!("Unknown ({})", ext),
        }
    } else {
        "Unknown".to_string()
    }
}

pub fn count_lines(content: &str) -> usize {
    content.lines().count()
}

