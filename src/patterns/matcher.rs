use std::path::Path;

use super::defaults::{CollapsePattern, PurposePattern};

/// Check if a directory name matches a collapse pattern
pub fn should_collapse<'a>(dir_name: &str, dir_path: &Path, patterns: &'a [CollapsePattern]) -> Option<&'a CollapsePattern> {
    for pattern in patterns {
        if dir_name == pattern.name {
            // If there's an indicator file required, check for it
            if let Some(indicator) = pattern.indicator {
                let indicator_path = dir_path.join(indicator);
                if indicator_path.exists() {
                    return Some(pattern);
                }
            } else {
                // No indicator required, match on name alone
                return Some(pattern);
            }
        }
    }
    None
}

/// Check if a filename matches any ignore pattern
pub fn should_ignore(filename: &str, patterns: &[&str]) -> bool {
    for pattern in patterns {
        if matches_glob(filename, pattern) {
            return true;
        }
    }
    false
}

/// Detect the purpose of a file based on its name
pub fn detect_purpose(filename: &str, patterns: &[PurposePattern]) -> Option<&'static str> {
    for pattern in patterns {
        if matches_glob(filename, pattern.pattern) {
            return Some(pattern.purpose);
        }
    }
    None
}

/// Simple glob matching (supports * wildcard)
fn matches_glob(text: &str, pattern: &str) -> bool {
    if pattern == text {
        return true;
    }

    if pattern.starts_with('*') && pattern.ends_with('*') {
        // *contains*
        let middle = &pattern[1..pattern.len() - 1];
        return text.contains(middle);
    }

    if pattern.starts_with('*') {
        // *suffix
        let suffix = &pattern[1..];
        return text.ends_with(suffix);
    }

    if pattern.ends_with('*') {
        // prefix*
        let prefix = &pattern[..pattern.len() - 1];
        return text.starts_with(prefix);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_glob() {
        assert!(matches_glob("test_foo.py", "test_*.py"));
        assert!(matches_glob("foo_test.py", "*_test.py"));
        assert!(matches_glob("main.py", "main.py"));
        assert!(matches_glob("foo.spec.ts", "*.spec.ts"));
        assert!(!matches_glob("main.rs", "main.py"));
        assert!(!matches_glob("test_foo.py", "*_test.py"));
    }

    #[test]
    fn test_should_ignore() {
        let patterns = vec![".DS_Store", "*.pyc", "*~"];
        assert!(should_ignore(".DS_Store", &patterns));
        assert!(should_ignore("foo.pyc", &patterns));
        assert!(should_ignore("backup~", &patterns));
        assert!(!should_ignore("main.py", &patterns));
    }
}
