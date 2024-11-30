//#![allow(unused_imports)]
use std::env;
use std::ffi::OsString;
use std::fs::{self};
use std::io::{self};
use std::path::Path;

mod format;
use format::DisplayOsString;

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
        format::format_if_dir(input_path).display()
    );

    let fmt_input: OsString = format::format_if_dir(&input_path);

    for i in get_children(input_path)?.iter() {
        let full_child_pathbuf = fmt_input.as_path().join(i.as_path());
        let full_child_path = Path::new(&full_child_pathbuf);

        /*
         * BUG (or bad quirk i guess):
         * it actually formats the child, but it includes the input_path :/
         * even if i use `.strip_prefix(input_path)?`, it gets rid of input_path, but then it doesn't format :(
         */

        let fmt_i = format::format_if_dir(full_child_path);
        println!("{}", &fmt_i.display());

        // println!("{:#?}", full_child_path.strip_prefix(input_path));
        // println!(
        //     "{:#?}",
        //     format_if_dir(full_child_path.strip_prefix(input_path)?)
        // );
    }

    Ok(())
}

fn get_children(parent: &Path) -> std::io::Result<Vec<OsString>> {
    if !parent.exists() {
        panic!("Path does not exist (b)");
    }

    let entries = fs::read_dir(parent)?;
    let mut children: Vec<OsString> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let child_name: OsString = entry.file_name();
        children.push(child_name);
    }

    Ok(children)
}
