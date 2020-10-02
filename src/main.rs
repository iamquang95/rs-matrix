use clap::{App, Arg};
use std::error::Error;
use std::str::FromStr;

fn main() {
    let config = extract_config().expect("Fail to parse params");
    println!("{:?}", config)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAlgoError {}

impl std::fmt::Display for ParseAlgoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided string was not `DFS` or `BFS`".fmt(f)
    }
}

impl Error for ParseAlgoError {}

#[derive(Copy, Clone, Debug)]
enum SearchAlgo {
    DFS,
    BFS,
}

impl FromStr for SearchAlgo {
    type Err = ParseAlgoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DFS" => Ok(SearchAlgo::DFS),
            "BFS" => Ok(SearchAlgo::BFS),
            _ => Err(ParseAlgoError {}),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Config {
    n_rows: usize,
    n_cols: usize,
    algo: SearchAlgo,
    step_delay: usize, // milliseconds
}

impl Config {
    fn new(n_rows: usize, n_cols: usize, algo: SearchAlgo, step_delay: usize) -> Config {
        Config {
            n_rows,
            n_cols,
            algo,
            step_delay,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            n_rows: 8,
            n_cols: 15,
            algo: SearchAlgo::BFS,
            step_delay: 80,
        }
    }
}

fn extract_config() -> Result<Config, Box<dyn Error>> {
    let matches = App::new("Rust Matrix")
        .arg(
            Arg::with_name("ROWS")
                .short("r")
                .long("rows")
                .help("Number of rows")
                .validator(is_number)
                .default_value("8"),
        )
        .arg(
            Arg::with_name("COLUMNS")
                .short("c")
                .long("columns")
                .help("Number of columns")
                .validator(is_number)
                .default_value("15"),
        )
        .arg(
            Arg::with_name("DELAY")
                .short("d")
                .long("delay")
                .help("Delay between two runs (in millisecond)")
                .validator(is_number)
                .default_value("80"),
        )
        .arg(
            Arg::with_name("ALGO")
                .short("a")
                .long("algo")
                .help("Search algorithm. Choose between DFS/BFS")
                .validator(is_algo)
                .default_value("BFS"),
        )
        .get_matches();
    Ok(Config::new(
        usize::from_str(matches.value_of("ROWS").unwrap())?,
        usize::from_str(matches.value_of("COLUMNS").unwrap())?,
        SearchAlgo::from_str(matches.value_of("ALGO").unwrap())?,
        usize::from_str(matches.value_of("DELAY").unwrap())?,
    ))
}

fn is_number(s: String) -> Result<(), String> {
    if usize::from_str(&s).is_ok() {
        Ok(())
    } else {
        Err(format!("{} is not a number", &s))
    }
}

fn is_algo(s: String) -> Result<(), String> {
    match s.to_uppercase().as_str() {
        "DFS" | "BFS" => Ok(()),
        _ => Err(format!("{} is not a supported algorithm", s)),
    }
}
