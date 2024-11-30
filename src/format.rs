use std::ffi::OsString;
use std::fs;
use std::path::Path;

// uncomment commented lines, and remove both `return`'s to instead return a `&Path`
// entry *needs* to be `&Path`
pub fn format_if_dir(entry: &Path) -> OsString {
    if entry.is_dir() && !ends_with_char(entry, '/') {
        // let fmt_dir: &Path = Path::new(format!("{}/", entry.display()));
        // let fmt_dir: OsString = match entry.to_str() {
        //     Some(i) => OsString::from(format!("{}/", i)),
        //     None => panic!(),
        // };
        //
        // fmt_dir.as_path()
        return OsString::from(format!("{}/", entry.display()));
    }

    // entry
    OsString::from(entry)
}

fn ends_with_char(dir: &Path, end_char: char) -> bool {
    let length = OsString::from(dir).len();
    let dir_as_bytes: &[u8] = match dir.to_str() {
        Some(i) => i.as_bytes(),
        None => panic!("Couldn't convert an OsString to &[u8]"),
    };

    dir_as_bytes[length - 1] as char == end_char
}

pub trait DisplayOsString {
    fn display(&self) -> &str;
    fn as_path(&self) -> &Path;
    // similar to `.unwrap_or_else()` for `Option<T>`'s
    // fn display_or_else<T>(&self, f: FnOnce() -> T) -> T;
}

impl DisplayOsString for OsString {
    // I just got tired of calling `x.to_str().unwrap()` a lot on `OsString`'s
    // I thought there were a lot more of those calls, but idk
    fn display(&self) -> &str {
        self.to_str()
            .unwrap_or_else(|| panic!("Couldn't convert OsString to &str"))
    }

    // solely to make `format_if_dir` work
    fn as_path(&self) -> &Path {
        // Path::new(self.as_os_str())
        Path::new(self)
    }
}
