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

    println!("Your input path was: {}", input_path.display());

    // begin CL arg testing
    match options.len() {
        0 => println!("No arguments found:"),
        1 => println!("1 argument found:"),
        i => println!("{} arguments found:", i),
    }

    println!("[{:#?}]\n", options.join(" "));
    // end CL arg testing

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

#[allow(dead_code)]
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
