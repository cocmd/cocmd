use crate::consts;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
/// Recursively scan all files in the specified directory and its subdirectories up to a given depth.
/// It looks for files with a name matching `source_label` and returns a list of paths to those files.
///
/// # Arguments
///
/// * `source_label` - The name of the file to search for in the directory and subdirectories.
/// * `scan_depth` - The maximum depth to recursively search for files.
///
/// # Returns
///
/// A vector of strings representing the paths to files that match `source_label`.
///
/// # Example
///
/// ```rust
/// let source_label = "/.../....";
/// let scan_depth = 2;
///
/// let paths = find_cocmd_files(source_label, scan_depth);
///
/// for path in paths {
///     println!("{}", path);
/// }
/// ```
///
/// This function is used to locate files with a specific name within a directory and its subdirectories.
///
/// Note: This function assumes that the current working directory is the starting point for the search.
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
