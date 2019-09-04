#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::path::Path;


fn main() {
    let matches = clap_app!(rsls => 
        (version: "0.1")
        (author: "Jesse Mazzella <jessemazzella@gmail.com>")
        (about: "Yet another 'ls' replacement written in Rust")
        (@arg path: -p --path +takes_value index(1) "The path to run against")
        (@arg all: -a --all "Do not ignore entries starting with '.'")
        (@arg reverse: -r --reverse "Reverse the order of the sort to get reverse lexicographical order")
        (@arg recurse: -R --recurse "Recursively list subdirectories encountered")
    ).get_matches();
    let path = matches.value_of("path").unwrap_or(".");
    let show_hidden = matches.is_present("all");
    let recurse = matches.is_present("recurse");
    let dir = list(Path::new(path), show_hidden, recurse);
    for entry in dir.entries {
        entry.print();
    }
}

struct Dir {
    name: String,
    entries: Vec<Dir>,
}

impl Dir {
    pub fn print(&self) -> () {
        self.printi(0);
    }
    fn printi(&self, indent: usize) -> () {
        println!("{: >width$}{}", "", self.name, width = indent);
        self.entries.iter().for_each(|item| {
            item.printi(indent + 2);
        });
    }
}

fn list(path: &Path, show_hidden: bool, recurse: bool) -> Dir {
    let mut subs = Vec::<Dir>::new();
    if path.is_dir() {
        let mut entries = std::fs::read_dir(path).unwrap();

        while let Some(entry) = entries.next() {
            match entry {
                Ok(entry) => {
                    let file_name = entry.file_name();
                    let file_name = file_name.to_str().take();
                    if !is_hidden(&file_name.unwrap()) || show_hidden {
                        if recurse {
                            subs.push(list(entry.path().as_path(), show_hidden, recurse));
                        } else {
                            subs.push(Dir {
                                name: entry.file_name().to_str().unwrap_or("unk2").to_string(),
                                entries: Vec::new(),
                            })
                        }
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
