use std::path::PathBuf;

use tokio::fs::DirEntry;

pub struct Path(String);

impl From<String> for Path {
    fn from(f: String) -> Self {
        Self(f)
    }
}

impl From<&PathBuf> for Path {
    fn from(f: &PathBuf) -> Self {
        let path = f.as_os_str().to_owned().to_string_lossy().to_string();
        Self(path)
    }
}

impl From<DirEntry> for Path {
    fn from(f: DirEntry) -> Self {
        Path::from(&f.path())
    }
}

impl From<Path> for String {
    fn from(f: Path) -> Self {
        f.0
    }
}
