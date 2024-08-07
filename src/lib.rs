use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type AppResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> AppResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(content) => {
                let mut index = 0;
                for val in content.lines() {
                    let line_text = val.unwrap();

                    let line_number = if config.number_lines
                        || (config.number_nonblank_lines && !line_text.is_empty())
                    {
                        index += 1;
                        index.to_string()
                    } else {
                        "".to_string()
                    };

                    let line = format!("{:>6} {}", line_number, line_text);
                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> AppResult<Config> {
    let matches = App::new("catrs")
        .version("0.1.0")
        .author("Kovacs Ervin <kov.ervin97@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
