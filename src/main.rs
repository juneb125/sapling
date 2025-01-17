use std::env;
use std::fs;
use std::io::{self, Result as IOResult, Write};
use std::path::{Path, PathBuf};

fn main() -> IOResult<()> {
    println!("Hello, world!");

    let mut stdout = io::stdout();
    let _ = stdout.lock();

    let cmd_line_input: Vec<String> = env::args().skip(1).collect();

    let input_path: &Path = Path::new(cmd_line_input.get(0).unwrap());

    // CL args
    // let options: &[String] = cmd_line_input.get(1..).unwrap();

    if !input_path.exists() {
        panic!("Input path does not exist");
    }

    if !input_path.is_dir() {
        panic!("Input path is not a directory");
    }

    // CL arg testing
    /*
    match options.len() {
        0 => println!("No arguments found"),
        1 => println!("1 argument found"),
        i => println!("{} arguments found", i),
    }
    */

    let children = get_children(input_path)?;
    let num_kids = children.len();

    writeln!(stdout, "{}", input_path.display())?;

    for (i, child) in children.iter().enumerate() {
        let display_child: &Path = child.strip_prefix(input_path).unwrap();

        let tree_prefix: &str = if i < (num_kids - 1) { "├" } else { "└" };

        if child.is_dir() {
            writeln!(stdout, "{}── {}/", tree_prefix, display_child.display())?;
        } else {
            writeln!(stdout, "{}── {}", tree_prefix, display_child.display())?;
        }
    }

    stdout.flush()
}

#[allow(dead_code)]
fn get_children(parent: &Path) -> IOResult<Vec<PathBuf>> {
    let entries = fs::read_dir(parent)?;
    let mut children: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let entry = entry?;
        children.push(entry.path());
    }

    Ok(children)
}
