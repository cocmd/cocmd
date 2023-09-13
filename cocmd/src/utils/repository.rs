use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::consts;

pub fn find_cocmd_files(source_label: &Path, scan_depth: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    // Define a helper function for recursion
    fn visit_dir(
        current_dir: &Path,
        scan_depth: usize,
        result: &mut Vec<String>,
        visited: &mut HashSet<PathBuf>,
    ) {
        if scan_depth == 0 || visited.contains(current_dir) {
            return;
        }

        visited.insert(current_dir.to_path_buf());

        if let Ok(entries) = fs::read_dir(current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        visit_dir(&path, scan_depth - 1, result, visited);
                    } else if path.is_file()
                        && path.file_name().unwrap_or_default() == consts::SOURCE_CONFIG_FILE
                    {
                        result.push(current_dir.to_string_lossy().into_owned());
                    }
                }
            }
        } else {
            print!("can't find path: {:?}", current_dir)
        }
    }

    visit_dir(source_label, scan_depth, &mut result, &mut visited);

    result
}
