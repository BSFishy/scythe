use std::env::var;
use std::path::{Path, PathBuf};

/// TODO: document this
trait HasTempDir {
    /// TODO: document this
    fn get_temp_dir() -> Option<PathBuf>;
}

/// TODO: document this function
pub fn get_temp_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        if let Ok(path) = var("TEMP") {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(path) = var("TMPDIR") {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else if cfg!(target_os = "linux") {
        Some(PathBuf::from("/tmp"))
    } else {
        None
    }
}

impl HasTempDir for Path {
    fn get_temp_dir() -> Option<PathBuf> {
        get_temp_dir()
    }
}

impl HasTempDir for PathBuf {
    fn get_temp_dir() -> Option<PathBuf> {
        get_temp_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_dir_works() {
        let path = get_temp_dir();

        if cfg!(any(target_os = "windows", target_os = "macos")) {
            assert!(path.is_some(), "should have gotten a path");

            let path = path.unwrap();
            assert!(path.exists(), "the path should exist");
            assert!(path.is_dir(), "the path should be a directory");
        } else if cfg!(target_os = "linux") {
            assert!(match path {
                Some(buf) => buf.to_str().unwrap() == "/tmp",
                None => false,
            }, "linux should return \"/tmp\"");
        } else {
            assert!(path.is_none(), "unknown platform should return none");
        }
    }

    #[test]
    fn path_temp_is_same() {
        assert_eq!(Path::get_temp_dir(), get_temp_dir(), "the path returned by Path should be the same as the function")
    }

    #[test]
    fn path_buf_temp_is_same() {
        assert_eq!(PathBuf::get_temp_dir(), get_temp_dir(), "the path returned by Path should be the same as the function")
    }
}
