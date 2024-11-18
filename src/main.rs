// #![allow(unused_imports)]
// use std::env::args;
use std::ffi::OsString;
use std::fs::{self, ReadDir};
use std::io::{self};
use std::path::Path;

fn main() -> io::Result<()> {
    let arguments: Vec<String> = std::env::args().collect();
    let input_path: &Path = Path::new(arguments.get(1).unwrap());

    if !input_path.exists() {
        panic!("Path does not exist (a)");
    }

    // println!("Your arguments were {:?}", arguments);

    println!("{:#?}'s formatted children:", input_path);

    for i in get_children(input_path)?.iter() {
        let i_as_path: &Path = Path::new(i);

        // ERROR: only formats if input_path is "./"
        match fmt_child_type(i_as_path) {
            Some(path) => println!("{}", path.to_str().unwrap()),
            None => panic!("Wasn't expecting this"),
        };
    }

    Ok(())
}

#[allow(dead_code)]
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

#[allow(dead_code)]
// entry *needs* to be &Path
// ERROR: only formats if input_path is "./"
fn fmt_child_type(entry: &Path) -> Option<OsString> {
    // this was making it panic even when the entry absolutely exists
    // if !entry.exists() {
    //     panic!("Path does not exist (c)");
    // }

    let mut name: OsString = match entry.file_stem() {
        Some(i) => i.to_os_string(),
        None => panic!("Could not read path"),
    };

    if entry.is_file() {
        return Some(entry.file_name().unwrap().to_os_string());
    } else if entry.is_dir() {
        name.push("/");
    } else if entry.is_symlink() {
        name.push("@");
    }

    Some(name)
}
