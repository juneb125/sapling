// #[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let arguments: Vec<String> = env::args().collect();
    let input_path: &Path = Path::new(arguments.get(1).unwrap());

    if !input_path.is_dir() {
        // not a directory :/
        panic!("Input path is not a directory")
    }

    let children = get_children(input_path);

    for i in children?.iter() {
        let display_i = match i.strip_prefix(input_path) {
            Ok(a) => a,
            Err(b) => panic!("{}", b),
        };

        if i.is_dir() {
            println!("{}/", display_i.display());
        } else {
            println!("{}", display_i.display());
        }
    }

    Ok(())
}

// #[allow(dead_code)]
fn get_children(parent: &Path) -> io::Result<Vec<PathBuf>> {
    if !parent.exists() {
        panic!("Path does not exist (b)");
    }

    let entries = fs::read_dir(parent)?;
    let mut children: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let entry = entry?;
        children.push(entry.path());
    }

    Ok(children)
}
