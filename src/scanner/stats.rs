use std::path::Path;
use walkdir::WalkDir;

/// Count files and directories within a path
pub fn count_dir_contents(path: &Path) -> (usize, usize) {
    let mut file_count = 0;
    let mut dir_count = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            file_count += 1;
        } else if entry.file_type().is_dir() && entry.path() != path {
            dir_count += 1;
        }
    }

    (file_count, dir_count)
}

/// Count lines in a directory (sum of all text files) - for future use
#[allow(dead_code)]
pub fn count_lines_in_dir(path: &Path) -> usize {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let text_extensions = [
        "py", "rs", "js", "ts", "jsx", "tsx", "go", "java", "kt", "rb", "php",
        "c", "cpp", "cc", "h", "hpp", "cs",
    ];

    let mut total = 0;

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let is_text = entry
            .path()
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| text_extensions.contains(&e))
            .unwrap_or(false);

        if is_text {
            if let Ok(file) = File::open(entry.path()) {
                total += BufReader::new(file).lines().count();
            }
        }
    }

    total
}
