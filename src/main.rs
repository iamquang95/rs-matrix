use clap::{App, Arg};
use std::error::Error;
use std::str::FromStr;

mod matrix;
mod solver;

use crossterm::{cursor, style, ExecutableCommand, QueueableCommand};
use std::io::Write;
use std::{thread, time::Duration};

fn main() {
    let mut stdout = std::io::stdout();
    stdout.execute(cursor::Hide).expect("failed to hide cursor");

    let config = extract_config().expect("Fail to parse params");

    let m = matrix::Matrix::new(config.n_rows, config.n_cols);
    draw_matrix(&mut stdout, &m);

    let traverse = |matrix: &matrix::Matrix, cell: &matrix::Cell| {
        if *cell != matrix.start && *cell != matrix.finish {
            thread::sleep(Duration::from_millis(config.step_delay as u64));
            draw_cell(&mut stdout, matrix, cell, style::Color::DarkYellow, '·')
        }
    };

    if config.algo == SearchAlgo::BFS {
        let mut bfs_solver = solver::BFSSolver::new(&m);
        let path_opt = bfs_solver.solve(traverse);
        if let Some(path) = path_opt {
            highlight_path(&mut stdout, &m, &path, config);
        }
    }
}

fn highlight_path(
    stdout: &mut std::io::Stdout,
    matrix: &matrix::Matrix,
    path: &Vec<matrix::Cell>,
    config: Config,
) {
    for cell in path.into_iter() {
        thread::sleep(Duration::from_millis(config.step_delay as u64));
        draw_cell(stdout, matrix, cell, style::Color::Green, 'x');
    }
}

fn draw_matrix(stdout: &mut std::io::Stdout, matrix: &matrix::Matrix) {
    for _ in 0..matrix.n_rows {
        stdout.write_fmt(format_args!("\r\n")).unwrap();
    }
    stdout.flush().unwrap();
    stdout
        .queue(cursor::MoveToPreviousLine(matrix.n_rows as u16))
        .unwrap();
    (*matrix.matrix)
        .into_iter()
        .enumerate()
        .for_each(|(r, row)| {
            row.into_iter().enumerate().for_each(|(c, cell)| {
                if *cell {
                    let cur_cell = matrix::Cell(r as isize, c as isize);
                    if cur_cell == matrix.start {
                        stdout
                            .queue(style::SetForegroundColor(style::Color::Green))
                            .unwrap();
                        stdout.write_fmt(format_args!("{}", 'S')).unwrap();
                    } else if cur_cell == matrix.finish {
                        stdout
                            .queue(style::SetForegroundColor(style::Color::Green))
                            .unwrap();
                        stdout.write_fmt(format_args!("{}", 'F')).unwrap();
                    } else {
                        stdout
                            .queue(style::SetForegroundColor(style::Color::Reset))
                            .unwrap();
                        stdout.write_fmt(format_args!("{}", ' ')).unwrap();
                    }
                } else {
                    stdout
                        .queue(style::SetForegroundColor(style::Color::DarkGrey))
                        .unwrap();
                    stdout.write_fmt(format_args!("{}", '█')).unwrap();
                };
            });
            stdout.queue(cursor::MoveToNextLine(1)).unwrap();
        });
    stdout.flush().unwrap();
}

fn draw_cell(
    stdout: &mut std::io::Stdout,
    matrix: &matrix::Matrix,
    cell: &matrix::Cell,
    color: style::Color,
    ch: char,
) {
    stdout.queue(cursor::SavePosition).unwrap();
    stdout
        .queue(cursor::MoveToPreviousLine(matrix.n_rows as u16))
        .unwrap();
    stdout.queue(cursor::MoveRight(cell.1 as u16)).unwrap();
    stdout.queue(cursor::MoveDown(cell.0 as u16)).unwrap();
    stdout.queue(style::SetForegroundColor(color)).unwrap();
    stdout.write_fmt(format_args!("{}", ch)).unwrap();
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.flush().unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAlgoError {}

impl std::fmt::Display for ParseAlgoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided string was not `DFS` or `BFS`".fmt(f)
    }
}

impl Error for ParseAlgoError {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
