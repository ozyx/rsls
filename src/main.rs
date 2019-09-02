extern crate clap;
use clap::{App, Arg};
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

fn main() {
    let matches = App::new("rsls")
        .version("0.1")
        .about("Yet another 'ls' replacement written in Rust")
        .author("Jesse Mazzella")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("The path to run against")
                .index(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Do not ignore entries starting with '.'"),
        )
        .get_matches();
    let path = matches.value_of("path").unwrap_or(".");
    let show_hidden = matches.is_present("all");
    list(Path::new(path), show_hidden);
}

fn list(path: &Path, show_hidden: bool) -> io::Result<()> {
    if path.is_dir() {
        let mut entries = fs::read_dir(path)?.peekable();

        while let Some(entry) = entries.next() {
            match entry?.file_name().to_str() {
                Some(file_name) => {
                    if !is_hidden(&file_name) || show_hidden {
                        if entries.peek().is_some() {
                            print!("{}  ", file_name);
                        } else {
                            print!("{}", file_name);
                        }
                    }
                }
                None => {}
            }
        }
    }
    Ok(())
}

fn is_hidden(file_name: &str) -> bool {
    return file_name.starts_with(".");
}
