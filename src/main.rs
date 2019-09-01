extern crate clap;
use clap::{App, Arg};

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
}
