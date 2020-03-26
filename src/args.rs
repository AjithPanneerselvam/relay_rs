use clap::{App, Arg};
use std::env;

pub struct Args {
    pub input_file: String,
    pub output_file: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("relay")
            .arg(Arg::with_name("input_file").help("Read from a file instead of stdin"))
            .arg(
                Arg::with_name("output_file")
                    .short("o")
                    .long("output_file")
                    .takes_value(true)
                    .help("Write output to file instead of stdout"),
            )
            .arg(
                Arg::with_name("silent")
                    .short("s")
                    .long("silent")
                    .help("silent the output of progress"),
            )
            .get_matches();

        let input_file = matches
            .value_of("input_file")
            .unwrap_or_default()
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .unwrap_or_default()
            .to_string();
        let silent = if matches.is_present("silent") {
            true
        } else {
            !env::var("SILENT").unwrap_or_default().is_empty()
        };

        Self {
            input_file,
            output_file,
            silent,
        }
    }
}
