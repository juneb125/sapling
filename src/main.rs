// #[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;

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
        println!("{}", i);
    }

    Ok(())
}

// #[allow(dead_code)]
fn get_children(parent: &Path) -> io::Result<Vec<String>> {
    if !parent.exists() {
        panic!("Path does not exist (b)");
    }

    let entries = fs::read_dir(parent)?;
    let mut children: Vec<String> = Vec::new();

    for entry in entries {
        let entry = entry?;

        match entry.path().strip_prefix(parent) {
            Ok(i) => {
                if i.is_dir() {
                    children.push(format!("|-- {}/", i.display()));
                } else {
                    children.push(format!("|-- {}", i.display()));
                }
            }
            Err(i) => panic!("{:#?}", i),
        }
    }

    Ok(children)
}
