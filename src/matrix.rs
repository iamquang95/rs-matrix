use rand::Rng;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_dirs() -> Vec<Direction> {
        vec!(Direction::Up, Direction::Down, Direction::Left, Direction::Right)
    }
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, Copy, PartialEq)]
pub struct Cell(isize, isize);

impl Cell {
    pub fn move_dir(&self, dir: Direction) -> Cell {
        match dir {
            Direction::Up => Cell(self.0 + 1, self.1),
            Direction::Down => Cell(self.0 - 1, self.1),
            Direction::Left => Cell(self.0, self.1 - 1),
            Direction::Right => Cell(self.0, self.1 + 1),
        }
    }
}

#[derive(Debug)]
pub struct Matrix {
    pub n_rows: usize,
    pub n_cols: usize,
    pub matrix: Vec<Vec<bool>>,
    pub start: Cell,
    pub finish: Cell,
}

impl Matrix {
    pub fn new(n_rows: usize, n_cols: usize) -> Matrix {
        let mut matrix = Vec::with_capacity(n_rows);
        let mut free_cells = Vec::new();
        let mut rng = rand::thread_rng();
        for r in 0..n_rows {
            let mut row = Vec::with_capacity(n_cols);
            for c in 0..n_cols {
                let is_free = if (r == 0 || r == n_rows - 1) || (c == 0 || c == n_cols - 1) {
                    false
                } else {
                    rng.gen_bool(2.0 / 3.0)
                };
                if is_free {
                    free_cells.push(Cell(r as isize, c as isize));
                }
                row.push(is_free);
            }
            matrix.push(row);
        }
        Matrix {
            n_rows,
            n_cols,
            matrix,
            start: free_cells[rng.gen_range(0, free_cells.len())],
            finish: free_cells[rng.gen_range(0, free_cells.len())],
        }
    }
    
    pub fn inside(&self, cell: Cell) -> bool {
        cell.0 >= 0 && (cell.0 as usize) < self.n_rows && cell.1 >= 0 && (cell.1 as usize) < self.n_cols
    }

    pub fn is_free_cell(&self, cell: Cell) -> bool {
        if !self.inside(cell) {
            return false;
        }
        if let Some(r) = self.matrix.get(cell.0 as usize) {
            if let Some(value) = r.get(cell.1 as usize) {
                *value
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(self.n_cols * self.n_cols + self.n_rows);
        (*self.matrix).into_iter().enumerate().for_each(|(r,row)| {
            row.into_iter().enumerate().for_each(|(c, cell)| {
                let c = if *cell {
                    let cur_cell = Cell(r as isize, c as isize);
                    if cur_cell == self.start {
                        'S'
                    } else if cur_cell == self.finish {
                        'F'
                    } else {
                    ' ' 
                    }
                } else { '*' };
                s.push(c);
            });
            s.push_str("\r\n");
        });
        write!(f, "{}", s)
    }
}
