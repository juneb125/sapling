#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let cmd_line_input: Vec<String> = env::args().collect();
    let input_path: &Path = Path::new(cmd_line_input.get(1).unwrap());

    // CL args
    let options: &[String] = cmd_line_input.get(2..).unwrap();

    if !input_path.exists() {
        panic!("Input path does not exist");
    }

    if !input_path.is_dir() {
        panic!("Input path is not a directory");
    }

    // CL arg testing
    match options.len() {
        0 => println!("No arguments found"),
        1 => println!("1 argument found"),
        i => println!("{} arguments found", i),
    }

    let children = get_children(input_path)?;
    let num_kids = children.len();

    println!("{}", input_path.display());

    for (i, child) in children.iter().enumerate() {
        let display_child: &Path = child.strip_prefix(input_path).unwrap();

        let tree_prefix: &str = if i < (num_kids - 1) { "├" } else { "└" };

        if child.is_dir() {
            println!("{}── {}/", tree_prefix, display_child.display());
        } else {
            println!("{}── {}", tree_prefix, display_child.display());
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn get_children(parent: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(parent)?;
    let mut children: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let entry = entry?;
        children.push(entry.path());
    }

    Ok(children)
}
