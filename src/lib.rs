use std::error::Error;
use clap::{App, Arg}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}

type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> AppResult<()> {
    println!("Hello, World!");
    Ok(())
}

pub fn get_args() -> AppResult<Config> {
    let matches = App::new("catrs")
        .version("0.1.0")
        .author("Kovacs Ervin <kov.ervin97@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
}
