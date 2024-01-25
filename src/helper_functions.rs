use std::fs;
use std::path::PathBuf;

pub fn has_write_permissions(path: &PathBuf) -> bool {
    let metadata = fs::metadata(path).unwrap();
    let permissions = metadata.permissions();
    !permissions.readonly()
}

pub fn can_read_directory(path: &PathBuf) -> bool {
    fs::read_dir(path).is_ok()
}
