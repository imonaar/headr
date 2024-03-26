use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(version = "0.0.1")]
#[command(author = "Kevin monari")]
#[command(about = "display first lines of a file", long_about=None)]

pub struct Config {
    #[arg(num_args = 0.., default_value = "-", help = "input file(s)", value_name = "FILE" )]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "lines",
        help = "Print count lines of each of the specified files.",
        default_value = "10",
        conflicts_with("bytes"),
        value_name = "LINES"
    )]
    lines: usize,

    #[arg(
        short = 'c',
        long = "bytes",
        help = "Print bytes of each of the specified files.",
        value_name = "BYTES"
    )]
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

