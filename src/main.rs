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
        .arg(
            Arg::with_name("recurse")
                .short("r")
                .long("recurse")
                .help("Print entries recursively"),
        )
        .get_matches();
    let path = matches.value_of("path").unwrap_or(".");
    let show_hidden = matches.is_present("all");
    let recurse = matches.is_present("recurse");
    let dir = list(Path::new(path), show_hidden, recurse);
    for entry in dir.entries {
        entry.print(recurse);
    }
}

struct Dir {
    name: String,
    entries: Vec<Dir>,
}

impl Dir {
    pub fn print(&self, recurse: bool) -> () {
        self.printi(0, recurse);
    }
    fn printi(&self, indent: usize, recurse: bool) -> () {
        println!("{: >width$}{}", "", self.name, width = indent);
        if recurse {
            self.entries.iter().for_each(|item| {
                item.printi(indent + 2, recurse);
            });
        }
    }
}

fn list(path: &Path, show_hidden: bool, recurse: bool) -> Dir {
    let mut subs = Vec::<Dir>::new();
    if path.is_dir() {
        let mut entries = fs::read_dir(path).unwrap();

        while let Some(entry) = entries.next() {
            match entry {
                Ok(entry) => {
                    let file_name = entry.file_name();
                    let file_name = file_name.to_str().take();
                    if !is_hidden(&file_name.unwrap()) || show_hidden {
                        subs.push(list(entry.path().as_path(), show_hidden, recurse));
                    }
                }
                Err(_) => {}
            }
        }
    }
    Dir {
        name: path.file_name().map_or("unk1".to_string(), |s| {
            s.to_str().unwrap_or("unk2").to_string()
        }),
        entries: subs,
    }
}

fn is_hidden(file_name: &str) -> bool {
    return file_name.starts_with(".");
}
