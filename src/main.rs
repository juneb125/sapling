use std::env;
use std::fs;
use std::io::{self, Result as IOResult, Write};
use std::path::{Path, PathBuf};

fn main() -> IOResult<()> {
    let mut stdout = io::stdout().lock();

    let argv: Vec<String> = env::args().skip(1).collect();
    let argc: usize = argv.len();

    if argc < 1 {
        panic!(
            "Not enough arguments supplied\n{} {}",
            "Expected 1 argument, found", argc
        );
    }

    let input_path: &Path = Path::new(&argv[0]);

    // CL args
    // let options: &[String] = argv[1..];

    if !input_path.exists() {
        panic!("Input path does not exist");
    }

    if !input_path.is_dir() {
        panic!("Input path is not a directory");
    }

    // uses the "argv, argc" naming convention
    let childv = get_children(input_path)?;
    let childc: usize = childv.len();

    writeln!(stdout, "{}", input_path.display())?;

    for (i, child) in childv.iter().enumerate() {
        let display_child: &Path = child.strip_prefix(input_path).unwrap();

        let tree_prefix: &str = match i {
            _ if i == (childc - 1) => "└",
            _ => "├",
        };

        if child.is_dir() {
            writeln!(stdout, "{tree_prefix}── {}/", display_child.display())?;
        } else {
            writeln!(stdout, "{tree_prefix}── {}", display_child.display())?
        }
    }

    stdout.flush()
}

fn get_children(parent: &Path) -> IOResult<Vec<PathBuf>> {
    let entries = fs::read_dir(parent)?;
    entries
        .map(|i| {
            // i: Result<DirEntry, Error>
            // j: DirEntry
            i.map(|j| j.path())
        })
        .collect()
}
