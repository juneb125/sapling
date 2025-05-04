use std::fs;
use std::io::Result as IOResult;
use std::path::{Path, PathBuf};

pub trait GetChildren {
    fn get_children(&self) -> IOResult<Vec<PathBuf>>;
}

impl GetChildren for Path {
    fn get_children(&self) -> IOResult<Vec<PathBuf>> {
        let entries = fs::read_dir(self)?;
        entries
            .map(|i| {
                // i: Result<DirEntry, Error>
                // j: DirEntry
                i.map(|j| j.path())
            })
            .collect()
    }
}

pub trait FormatPath {
    fn fmt(&self) -> String;
}

impl FormatPath for Path {
    fn fmt(&self) -> String {
        match self {
            i if i.is_dir() => format!("{}/", self.display()),
            _ => format!("{}", self.display()),
        }
    }
}

pub mod box_chars {
    pub const TEE: &'static str = "├";
    pub const ELL: &'static str = "└";
    pub const VERT: &'static str = "│";
    pub const ACROSS: &'static str = "─";
    pub const DBL_ACROSS: &'static str = "──";
}
