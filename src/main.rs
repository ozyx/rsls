extern crate clap;
use clap::{App, Arg};
use std::path::Path;
use std::fs::{self};
use std::io;

fn main() {
    let matches = App::new("rsls")
                    .version("0.1")
                    .about("Yet another 'ls' replacement written in Rust")
                    .author("Jesse Mazzella")
                    .arg(Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .help("The path to run against")
                        .index(1)
                        .takes_value(true))
                    .get_matches();
    
    let path = matches.value_of("path").unwrap_or(".");
    list(Path::new(path));
}

fn list(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            println!("{}", entry.file_name().to_str().unwrap());
        }
    }
    Ok(())
}