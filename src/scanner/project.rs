use std::fs;
use std::path::Path;

use crate::map::ProjectType;
use crate::patterns::default_project_patterns;

/// Detect the project type from marker files
pub fn detect_project_type(root: &Path) -> ProjectType {
    let patterns = default_project_patterns();
    let mut detected = ProjectType::default();
    let mut found_languages: Vec<String> = Vec::new();

    for pattern in &patterns {
        for marker in pattern.markers {
            let marker_path = root.join(marker);
            if marker_path.exists() {
                let lang = pattern.language.to_string();
                
                // Track all languages found
                if !found_languages.contains(&lang) {
                    found_languages.push(lang);
                }
                
                detected.detected_from.push(marker.to_string());

                // Try to detect framework from the marker file contents
                if detected.framework.is_none() {
                    if let Some(framework) = detect_framework(&marker_path, pattern.frameworks) {
                        detected.framework = Some(framework);
                    }
                }
            }
        }
    }

    // Handle TypeScript special case (often alongside JavaScript)
    let has_tsconfig = root.join("tsconfig.json").exists();
    if has_tsconfig {
        if !found_languages.contains(&"typescript".to_string()) {
            // If JS was first, replace it with TS as primary
            if let Some(pos) = found_languages.iter().position(|l| l == "javascript") {
                found_languages[pos] = "typescript".to_string();
            } else {
                found_languages.push("typescript".to_string());
            }
        }
        if !detected.detected_from.contains(&"tsconfig.json".to_string()) {
            detected.detected_from.push("tsconfig.json".to_string());
        }
    }

    detected.languages = found_languages;
    detected
}

/// Try to detect framework from file contents
fn detect_framework(marker_path: &Path, frameworks: &[(&str, &str)]) -> Option<String> {
    if frameworks.is_empty() {
        return None;
    }

    let contents = fs::read_to_string(marker_path).ok()?;
    let contents_lower = contents.to_lowercase();

    for (framework_name, package_hint) in frameworks {
        // Simple check: does the file contain the package name?
        if contents_lower.contains(&package_hint.to_lowercase()) {
            return Some(framework_name.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_detect_python_project() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("pyproject.toml"), "[project]\nname = \"test\"").unwrap();
        
        let detected = detect_project_type(temp.path());
        assert_eq!(detected.languages.first(), Some(&"python".to_string()));
    }

    #[test]
    fn test_detect_rust_project() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();
        
        let detected = detect_project_type(temp.path());
        assert_eq!(detected.languages.first(), Some(&"rust".to_string()));
    }

    #[test]
    fn test_detect_flask_framework() {
        let temp = TempDir::new().unwrap();
        fs::write(
            temp.path().join("pyproject.toml"),
            "[project]\ndependencies = [\"flask\"]"
        ).unwrap();
        
        let detected = detect_project_type(temp.path());
        assert_eq!(detected.languages.first(), Some(&"python".to_string()));
        assert_eq!(detected.framework, Some("flask".to_string()));
    }
}
