use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Alejandro Martinez <amnaredo@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("lines")
                .takes_value(false)
                .help("Show line count")
                .short("l")
                .long("lines"),
        )
        .arg(
            Arg::with_name("words")
                .takes_value(false)
                .help("Show word count")
                .short("w")
                .long("words"),
        )
        .arg(
            Arg::with_name("bytes")
                .takes_value(false)
                .help("Show byte count")
                .short("c")
                .long("bytes")
                .conflicts_with("chars"),
        )
        .arg(
            Arg::with_name("chars")
                .takes_value(false)
                .help("Show character")
                .short("m")
                .long("chars")
                .conflicts_with("bytes"),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
        chars = false;
    }
    
    Ok(Config {
        files:  matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn read_whole_lines(file: &mut Box<dyn BufRead>) -> MyResult<Vec<String>> {

    let mut lines = vec!();

    // read lines
    loop {
        let mut line = String::new();
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 { break; }
        lines.push(line);
    }

    Ok(lines)
}

pub fn run(config: Config) -> MyResult<()> {
  
    let mut total_lines_count = 0;
    let mut total_words_count = 0;
    let mut total_bytes_count = 0;
    let mut total_chars_count = 0;
    
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                
                // NOTE about lines() iterator:
                // Each string returned will not have a newline byte (the 0xA byte)
                // or CRLF (0xD, 0xA bytes) at the end.

                // let lines: Vec<String> = file
                //             .lines()
                //             .map(|result| match result {
                //                 Ok(line) => line,
                //                 Err(_) => String::new(),
                //             })
                //             .collect();

                let lines = read_whole_lines(&mut file).unwrap();

                if config.lines {
                    let lines_count = lines.len();
                    print!("{:>6}", lines_count);

                    total_lines_count += lines_count;
                }
                
                if config.words {
                    let words_count: usize = lines
                                .iter()
                                .map(|line| line.split_ascii_whitespace().count())
                                .sum();
                    print!("{:>6}", words_count);

                    total_words_count += words_count;

                }
                
                if config.bytes {
                    let bytes_count: usize = lines
                                .iter()
                                .map(|line| line.bytes().count())
                                .sum();
                    print!("{:>6}", bytes_count);

                    total_bytes_count += bytes_count;
                }

                if config.chars {
                    let chars_count: usize = lines
                                .iter()
                                .map(|line| line.chars().count())
                                .sum();
                    print!("{:>6}", chars_count);

                    total_chars_count += chars_count;
                }
                println!(" {}", filename);
            }
        }
    }

    if config.files.len() > 1 {
        if config.lines { print!("{:>6}", total_lines_count)}
        if config.words { print!("{:>6}", total_words_count)}
        if config.bytes { print!("{:>6}", total_bytes_count)}
        if config.chars { print!("{:>6}", total_chars_count)}
        println!(" total");
    }

    Ok(())
}
