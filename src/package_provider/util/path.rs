use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn extract_local_path(package: &String) -> Option<PathBuf> {
    // find out if package is a local path, even if it doesn't exist
    // if it is, return Some(path)
    // otherwise return None
    let path = Path::new(package);

    // check for local file system pattern
    if path.is_absolute() {
        return Some(path.to_path_buf());
    }

    if path.is_relative() {
        // return the absolute path
        // fetch the current working directory and join with path
        let cwd = std::env::current_dir().unwrap();
        let abs_path = cwd.join(path);

        // Check if the path exists
        if abs_path.exists() {
            // Canonicalize the path to get the absolute path
            if let Ok(canonical_path) = fs::canonicalize(abs_path) {
                // Convert the result to a PathBuf
                let result_path: PathBuf = canonical_path;
                // println!("{:?}", result_path);
                return Some(result_path);
            }
        }
    }

    None
}
