// #![allow(unused_imports)]
use std::ffi::OsString;
use std::fs::{self, ReadDir};
// use std::io::Result;
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let current_dir: &Path = Path::new(".");

    for i in get_children(current_dir)?.iter() {
        println!("{:#?}", i.as_os_str().to_str().unwrap())
    }

    Ok(())
}

fn get_children(dir: &Path) -> std::io::Result<Vec<OsString>> {
    if !dir.exists() {
        panic!("directory does not exist");
    }

    let entries: ReadDir = fs::read_dir(dir)?;
    let mut children: Vec<OsString> = Vec::new();
    for entry in entries {
        let entry = entry?;
        let dir_name: OsString = entry.file_name();
        children.push(dir_name)
    }
    Ok(children)
}
