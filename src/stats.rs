use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
pub struct Stats {
    pub language_files: HashMap<String, usize>,
    pub language_lines: HashMap<String, usize>,
    pub total_files: usize,
    pub total_lines: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            language_files: HashMap::new(),
            language_lines: HashMap::new(),
            total_files: 0,
            total_lines: 0,
        }
    }

    pub fn add_file(&mut self, language: &str, line_count: usize) {
        *self.language_files.entry(language.to_string()).or_insert(0) += 1;
        *self.language_lines.entry(language.to_string()).or_insert(0) += line_count;
        self.total_files += 1;
        self.total_lines += line_count;
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Language Statistics:")?;
        writeln!(f, "-------------------")?;
        
        // Get sorted languages by file count
        let mut languages: Vec<_> = self.language_files.keys().collect();
        languages.sort();
        
        // Display table header
        writeln!(f, "Language      | Files | Lines    | % of Lines")?;
        writeln!(f, "--------------|-------|----------|----------")?;
        
        // Display stats for each language
        for lang in languages {
            let files = self.language_files.get(lang).unwrap_or(&0);
            let lines = self.language_lines.get(lang).unwrap_or(&0);
            let percentage = if self.total_lines > 0 {
                (*lines as f64 / self.total_lines as f64) * 100.0
            } else {
                0.0
            };
            
            writeln!(
                f,
                "{:<14} | {:5} | {:8} | {:6.2}%",
                lang, files, lines, percentage
            )?;
        }
        
        // Display totals
        writeln!(f, "--------------|-------|----------|----------")?;
        writeln!(
            f,
            "{:<14} | {:5} | {:8} | 100.00%",
            "Total", self.total_files, self.total_lines
        )?;
        
        Ok(())
    }
}

