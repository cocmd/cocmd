use std::path::Path;

pub fn extract_local_path(source: &String) -> Option<&Path> {
    let path = Path::new(source);

    if path.exists() {
        return Some(path);
    }

    None
}
