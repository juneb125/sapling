use std::env;
use std::io::{self, Result as IOResult, Write};
use std::path::{Path, PathBuf};

mod util;

use util::{box_chars, FormatPath, GetChildren};

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
    } else if !input_path.is_dir() {
        panic!("Input path is not a directory");
    }

    // uses the "argv, argc" naming convention
    let childv: Vec<PathBuf> = input_path.get_children()?;
    let childc: usize = childv.len();

    writeln!(stdout, "{}", input_path.display())?;

    for (i, child) in childv.iter().enumerate() {
        let display_child: &Path = child
            .strip_prefix(input_path)
            .expect("Couldn't strip prefix");

        let tree_prefix = match i {
            _ if i == (childc - 1) => box_chars::ELL,
            _ => box_chars::TEE,
        };

        writeln!(
            stdout,
            "{tree_prefix}{} {}",
            box_chars::DBL_ACROSS,
            display_child.fmt_path()
        )?;
    }

    stdout.flush()
}
