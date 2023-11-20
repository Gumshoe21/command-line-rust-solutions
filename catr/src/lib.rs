use clap;
use std::error::Error; // import the Error trait
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)] // The derive macro adds the Debug trait so the struct can be printed.
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    println!("Hello, world!");
}

// pub fn run(config: Config) -> MyResult<()> {
//     for filename in config.files {
//         println!("{}", filename);
//     }
//     Ok(())
// }

// public function named run that accepts a Config option and returns a Result containing a Unit type
pub fn run(config: Config) -> MyResult<()> {
    // for each filename within the files property of the supplied Config object
    for filename in config.files {
        // open each filename and match each corresponding state (either error or ok) with the appropriate actions
        match open(&filename) {
            // if we have an error, eprintln! it
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            // if everything's okay, handle it
            Ok(file) => {
                // declare a mutable "last_num"
                let mut last_num = 0;
                // for each file's lines, extract the line itself plus a line_num which will act as the line number
                // Iterator::enumerate : This method will return a tuple containing the index position
                // and value for each element in an iterable, which is something
                // that can produce values until exhausted:
                for (line_num, line) in file.lines().enumerate() {
                    // unpack the line
                    let line = line?;
                    // if our command indicated that we want to number our lines with -n or --number,
                    // print each line wth the line number (starting at line 1, hence the + 1 as line_num = 0 to start) and the line itself
                    if config.number_lines {
                        // (You can use < for left-justified and ^ for centered text.)
                        // This syntax is similar to printf in C, Perl, and Python’s string formatting.
                        println!("{:>6}\t{}", line_num + 1, line);
                        // otherwise, if we only have -b or --number-nonblank, then number only the non-blank lines
                    } else if config.number_nonblank_lines {
                        // if the line isn't completely empty (aka it's not blank), then increment last_num so that we may print
                        // it next to the non-blank line. we incrememnt only when there's a non-blank line because we only want to count the non-blank ones.
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            // otherwise, if it's a blank line, just print a blank line
                            println!();
                        }
                    } else {
                        // otherwise, if there are no passed in options, print the line as normal unnumbered
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = clap::App::new("catr")
        .version("0.1.0")
        .author("Matthew Smilansky <msmilansky@gmail.com")
        .about("Rust cat")
        // What goes here?
        .arg(
            clap::Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            clap::Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            clap::Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();
    // A lossy value is one where if it contains invalid UTF-8 code points, those invalid points will be replaced with \u{FFFD}
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
