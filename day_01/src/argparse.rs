use clap::{crate_description, crate_name, crate_version, value_parser, Arg, ArgMatches, ColorChoice, Command};
use std::env;
use std::path::PathBuf;

pub fn get_args() -> ArgMatches {
    let app = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .color(ColorChoice::Auto)
        .max_term_width(100)
        .arg(
            Arg::new("FILE")
                .short('i')
                .help("File to read, use '-' for standard input")
                .value_parser(value_parser!(PathBuf)),
        );
    app.get_matches_from(env::args().collect::<Vec<String>>())
}
