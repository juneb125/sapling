// #![allow(unused_imports)]
use std::ffi::OsString;
use std::fs::{self, ReadDir};
use std::io::{self};
use std::path::Path;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    println!("What directory?");

    let mut input = String::new();
    stdin.read_line(&mut input)?;

    let input_dir: &Path = Path::new(input.trim());

    if !input_dir.exists() {
        panic!("Directory does not exist");
    }

    let dir_str = &input_dir.to_str().unwrap();

    println!("{}'s children:", dir_str);

    for i in get_children(input_dir)?.iter() {
        println!("{}", i.to_str().unwrap())
    }

    Ok(())
}

fn get_children(dir: &Path) -> io::Result<Vec<OsString>> {
    if !dir.exists() {
        panic!("Path does not exist");
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
