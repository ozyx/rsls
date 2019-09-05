use std::path::{PathBuf};

pub struct Options {
    pub path: PathBuf,
    pub show_hidden: bool,
    pub recurse: bool,
    pub reverse: bool,
}

impl Options {
    pub fn from_args(args: clap::ArgMatches<'_>) -> Options {
        Options {
            path: PathBuf::from(args.value_of("path").unwrap_or(".")),
            show_hidden: args.is_present("all"),
            recurse: args.is_present("recurse"),
            reverse: args.is_present("reverse")
        }
    }
}