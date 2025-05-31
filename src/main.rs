use std::env;
use std::io::{self, Result as IOResult, Write};
use std::path::{Path, PathBuf};

mod util;

use util::{box_chars, FormatPath, GetChildren};

fn main() -> IOResult<()> {
    let mut stdout = io::stdout().lock();

    let argv: Vec<String> = env::args().skip(1).collect();
    let argc: usize = argv.len();

    panic_if!(
        argc < 1,
        "Not enough arguments supplied, expected 1, found {argc}"
    );

    let input_path: &Path = Path::new(&argv[0]);

    panic_if!(!input_path.exists(), "Input path does not exist");
    panic_if!(!input_path.is_dir(), "Input path is not a directory");

    writeln!(stdout, "{}/", input_path.display())?;

    // uses the "argv, argc" naming convention
    let childv: Vec<PathBuf> = input_path.get_children()?;
    let childc: usize = childv.len();

    for (i, child) in childv.iter().enumerate() {
        let tree_prefix = match i {
            _ if i == (childc - 1) => box_chars::ELL,
            _ => box_chars::TEE,
        };

        writeln!(
            stdout,
            "{tree_prefix}{} {}",
            box_chars::DBL_ACROSS,
            child.fmt_path(input_path)
        )?;
    }

    stdout.flush()
}

/// more-or-less shorthand for:
/// ```
/// panic_if!(condition, message)
///
/// // turns into...
/// if condition {
///     panic!("{}", message)
/// }
/// ```
#[macro_export]
macro_rules! panic_if {
    ($condition:expr) => {
        let c: bool = $condition;
        match c {
            true => panic!(),
            false => (),
        }
    };
    ($condition:expr, $($msg:tt)+) => {
        let c: bool = $condition;
        match c {
            true => panic!("{}", format_args!($($msg)+)),
            false => (),
        }
    };
}
