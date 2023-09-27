use std::path::Path;

pub fn extract_local_path(source: &String) -> Option<&Path> {
    // find out if source is a local path, even if it doesn't exist
    // if it is, return Some(path)
    // otherwise return None
    let path = Path::new(source);

    // check for local file system pattern
    if path.is_absolute() {
        return Some(path);
    }

    None
}
