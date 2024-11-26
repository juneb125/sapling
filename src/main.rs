// #![allow(unused_imports)]
use std::env;
use std::ffi::OsString;
use std::fs::{self, ReadDir};
use std::io::{self};
use std::path::Path;

fn main() -> io::Result<()> {
    let arguments: Vec<String> = env::args().collect();
    let input_path: &Path = Path::new(arguments.get(1).unwrap());

    match arguments.len() {
        0 => panic!("What the hell?!"),
        1 => {
            panic!("Please enter the path you want to apply to this program");
            // ask for input again instead of panicking???
        }
        2 => (),
        3.. => panic!("Unexpected number of arguments\nPlease enter just the path"),
    }

    if !input_path.exists() {
        panic!("Path does not exist (a)");
    }

    println!(
        "Your path formatted is: {}",
        format_if_dir(input_path).display()
    );

    let fmt_input: OsString = format_if_dir(&input_path);

    for i in get_children(input_path)?.iter() {
        let full_child_pathbuf = fmt_input.as_path().join(i.as_path());
        let full_child_path = Path::new(&full_child_pathbuf);

        /*
         * BUG (or bad quirk i guess):
         * it actually formats the child, but it includes the input_path :/
         * even if i use `.strip_prefix(input_path)?`, it gets rid of input_path, but then it doesn't format :(
         */

        let fmt_i = format_if_dir(full_child_path);
        println!("{}", &fmt_i.display());

        // println!("{:#?}", full_child_path.strip_prefix(input_path));
        // println!(
        //     "{:#?}",
        //     format_if_dir(full_child_path.strip_prefix(input_path)?)
        // );
    }

    Ok(())
}

fn get_children(parent: &Path) -> io::Result<Vec<OsString>> {
    if !parent.exists() {
        panic!("Path does not exist (b)");
    }

    let entries: ReadDir = fs::read_dir(parent)?;
    let mut children: Vec<OsString> = Vec::new();
    for entry in entries {
        let entry = entry?;
        let child_name: OsString = entry.file_name();
        children.push(child_name);
    }

    Ok(children)
}

// uncomment commented lines, and remove both `return`'s to instead return a `&Path`
// entry *needs* to be `&Path`
fn format_if_dir(entry: &Path) -> OsString {
    if entry.is_dir() && !ends_with_char(entry, '/') {
        // let fmt_dir: &Path = Path::new(format!("{}/", entry.display()));
        // let fmt_dir: OsString = match entry.to_str() {
        //     Some(i) => OsString::from(format!("{}/", i)),
        //     None => panic!(),
        // };
        //
        // fmt_dir.as_path()
        return OsString::from(format!("{}/", entry.display()));
    }

    // entry
    OsString::from(entry)
}

fn ends_with_char(dir: &Path, end_char: char) -> bool {
    let length = OsString::from(dir).len();
    let dir_as_bytes: &[u8] = match dir.to_str() {
        Some(i) => i.as_bytes(),
        None => panic!("Couldn't convert an OsString to &[u8]"),
    };

    dir_as_bytes[length - 1] as char == end_char
}

trait DisplayOsString {
    fn display(&self) -> &str;
    fn as_path(&self) -> &Path;
    // similar to `.unwrap_or_else()` for `Option<T>`'s
    // fn display_or_else<T>(&self, f: FnOnce() -> T) -> T;
}

impl DisplayOsString for OsString {
    // I just got tired of calling `x.to_str().unwrap()` a lot on `OsString`'s
    // I thought there were a lot more of those calls, but idk
    fn display(&self) -> &str {
        self.to_str()
            .unwrap_or_else(|| panic!("Couldn't convert OsString to &str"))
    }

    // solely to make `format_if_dir` work
    fn as_path(&self) -> &Path {
        // Path::new(self.as_os_str())
        Path::new(self)
    }
}
