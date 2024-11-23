// #![allow(unused_imports)]
use std::env;
use std::ffi::OsString;
use std::fs::{self, ReadDir};
use std::io::{self};
use std::path::Path;

fn main() -> io::Result<()> {
    let arguments: Vec<String> = env::args().collect();
    let input_path: &Path = Path::new(arguments.get(1).unwrap());

    if !input_path.exists() {
        panic!("Path does not exist (a)");
    }

    if arguments.len() != 2 {
        panic!("Unexpected number of arguments.\nPlease enter just the path");
    }

    println!("{}'s formatted children:", input_path.display());

    for i in get_children(input_path)?.iter() {
        let i_as_path: &Path = Path::new(i);

        /* BUG: only formats if input_path is "./"
         * maybe because it's not the full relative path (just the child's relative path to its parent?????)
         */
        println!("{}", format_path(i_as_path).to_str().unwrap());
    }

    Ok(())
}

// #[allow(dead_code)]
fn get_children(parent: &Path) -> io::Result<Vec<OsString>> {
    if !parent.exists() {
        panic!("Path does not exist (b)");
    }

    let entries: ReadDir = fs::read_dir(parent)?;
    let mut children: Vec<OsString> = Vec::new();
    for entry in entries {
        let entry = entry?;
        let child_name: OsString = entry.file_name();
        children.push(child_name);
    }

    Ok(children)
}

// entry *needs* to be &Path
fn format_path(entry: &Path) -> OsString {
    if entry.is_dir() && !ends_with_char(entry, '/') {
        let fmt_dir: String = format!("{}/", entry.display());
        return OsString::from(fmt_dir);
    }

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
