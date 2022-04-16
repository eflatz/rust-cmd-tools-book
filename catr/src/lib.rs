use std::error::Error;
use clap::{Command, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
    .version("0.1.0")
    .author("Emilio<eflatz@arhitekt.hr>")
    .about("Rust cat")
    .arg(
        Arg::new("files")
        .value_name("file")
        .help("INPUT TEXT")
        .multiple_occurrences(true)
        .default_value("-")
        .allow_invalid_utf8(true)
    )
    .arg(
        Arg::new("number_lines")
        .short('n')
        .help("Do not print new line")
        .takes_value(false)
        .conflicts_with("number_nonblank_lines")
    )
    .arg(
        Arg::new("number_nonblank_lines")
        .short('b')
        .help("Do not number blank lines")
        .takes_value(false)
    )
    .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines")
    })

}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        println!("{}", filename)
    }
    Ok(())
}