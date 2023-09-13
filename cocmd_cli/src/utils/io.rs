use std::fs;
use std::io::{Write, BufRead};
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;


/// Normalizes a path by joining it with a base path if provided and resolving to an absolute path.
///
/// # Arguments
///
/// - `relative_path`: The relative path to normalize.
/// - `base_path`: An optional base path to join with the relative path.
///
/// # Returns
///
/// A normalized absolute path as a `String`.
pub fn normalize_path(relative_path: &str, base_path: Option<&str>) -> String {
    if let Some(base) = base_path {
        // Join the base path with the relative path and then resolve to an absolute path
        let abs_path = Path::new(base).join(relative_path).canonicalize();
        if let Ok(path) = abs_path {
            return path.to_string_lossy().into_owned();
        }
    }

    // Resolve the relative path to an absolute path using the current working directory
    if let Ok(abs_path) = Path::new(relative_path).canonicalize() {
        return abs_path.to_string_lossy().into_owned();
    }

    // Return the original path if normalization fails
    relative_path.to_owned()
}

/// Checks if a file or directory exists.
///
/// # Arguments
///
/// - `path`: The path to check for existence.
///
/// # Returns
///
/// `true` if the path exists, otherwise `false`.
pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Creates a directory and its parents if necessary.
///
/// # Arguments
///
/// - `dir`: The directory path to create.
pub fn mkdir(dir: &str) {
    if let Err(_) = fs::create_dir_all(dir) {
        eprintln!("Failed to create directory: {}", dir);
    }
}

/// Creates an empty file.
///
/// # Arguments
///
/// - `file`: The file path to create.
pub fn touch(file: &str) {
    if let Err(_) = fs::File::create(file) {
        eprintln!("Failed to create file: {}", file);
    }
}

/// Reads lines from a file into a `Vec<String>`.
///
/// # Arguments
///
/// - `file`: The file path to read.
///
/// # Returns
///
/// A vector of strings containing the lines from the file.
pub fn file_read_lines(file: &str) -> Result<Vec<String>, std::io::Error> {
    let mut lines = Vec::new();
    let file = fs::File::open(file)?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

/// Writes lines to a file.
///
/// # Arguments
///
/// - `file`: The file path to write to.
/// - `lines`: A vector of strings to write to the file.
pub fn file_write_lines(file: &str, lines: &[String]) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(file)?;
    for line in lines {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

/// Writes content to a file.
///
/// # Arguments
///
/// - `file`: The file path to write to.
/// - `content`: The content to write to the file.
pub fn file_write(file: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(file, content)
}

/// Gets a temporary file.
///
/// # Returns
///
/// A temporary file represented as a `std::fs::File`.
pub fn get_tmp_file() -> Result<std::fs::File, std::io::Error> {
    let tmp_dir = std::env::temp_dir();
    let mut tmp_file = tmp_dir.clone();
    tmp_file.push("tempfile"); // You can specify the filename here
    fs::File::create(tmp_file)
}

/// Gets the path to the system's temporary directory.
///
/// # Returns
///
/// The path to the temporary directory as a `PathBuf`.
pub fn get_tmp() -> PathBuf {
    std::env::temp_dir()
}

/// Changes the file mode to be executable.
///
/// # Arguments
///
/// - `file`: The file path to modify.
pub fn chmod_x(file: &str) {
    fs::set_permissions(file, fs::Permissions::from_mode(0o755)).unwrap();
}




// Function to serialize a value to YAML and write it to a file
pub fn to_yaml_file<T>(data: &T, file: &str) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    let mut file = File::create(file)?;
    serde_yaml::to_writer(&mut file, data)?;
    Ok(())
}

// Function to deserialize a value from a YAML file
pub fn from_yaml_file<T>(file: &str) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    // Open the file and deserialize directly from it
    let file = File::open(file)?;
    let result = serde_yaml::from_reader(file)?;

    Ok(result)
}

pub fn from_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the entire contents of the file into a String
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}