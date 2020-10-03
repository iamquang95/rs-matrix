use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, Hash, Eq, Copy, PartialEq)]
struct Cell(usize, usize);

#[derive(Debug)]
pub struct Matrix {
    n_rows: usize,
    n_cols: usize,
    matrix: Vec<Vec<bool>>,
    start: Cell,
    finish: Cell,
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
                    free_cells.push(Cell(r, c));
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
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(self.n_cols * self.n_cols + self.n_rows);
        (*self.matrix).into_iter().for_each(|row| {
            row.into_iter().for_each(|cell| {
                let c = if *cell { ' ' } else { '*' };
                s.push(c);
            });
            s.push_str("\r\n");
        });
        write!(f, "{}", s)
    }
}
