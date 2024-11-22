// #![allow(unused_imports)]
use std::env;
use std::ffi::OsString;
use std::fs::{self, ReadDir};
use std::io::{self};
use std::path::Path;

fn main() -> io::Result<()> {
    let arguments: Vec<String> = env::args().collect();
    let input_path: &Path = Path::new(arguments.get(1).unwrap());
    // let input_path: Vec<&Path> = arguments[1..].iter().map(|a| Path::new(a)).collect();

    // if !input_path.exists() {
    //     panic!("Path does not exist (a)");
    // }

    // println!("Your arguments were {:?}", arguments);

    println!("{:#?}'s formatted children:", input_path);

    for i in get_children(input_path)?.iter() {
        let i_as_path: &Path = Path::new(i);

        /*
         * BUG: only formats if input_path is "./"
         * maybe because it's not the full relative path (just the child's relative path to its parent?????)
         */
        println!("{:#?}", format_pls(i_as_path));
    }

    Ok(())
}

// #[allow(dead_code)]
fn get_children(dir: &Path) -> io::Result<Vec<OsString>> {
    if !dir.exists() {
        panic!("Path does not exist (b)");
    }

    let entries: ReadDir = fs::read_dir(dir)?;
    let mut children: Vec<OsString> = Vec::new();
    for entry in entries {
        let entry = entry?;
        let dir_name: OsString = entry.file_name();
        children.push(dir_name);
    }

    Ok(children)
}

// entry *needs* to be &Path
fn format_pls(entry: &Path) -> OsString {
    if entry.is_dir() && !ends_with(entry, '/') {
        let fmt_dir = format!("{}/", entry.display());
        return OsString::from(fmt_dir);
    }

    OsString::from(entry)
}

fn ends_with(dir: &Path, foo: char) -> bool {
    let length = OsString::from(dir).len();
    let bar = match dir.to_str() {
        Some(i) => i.as_bytes(),
        None => panic!("Couldn't convert to an OsString"),
    };

    bar[length - 1] as char == foo
}
