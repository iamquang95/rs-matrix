use crate::matrix::{Cell, Direction, Matrix};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct BFSSolver<'s> {
    matrix: &'s Matrix,
    path: Option<Vec<Cell>>,
}

impl<'s> BFSSolver<'s> {
    pub fn new(matrix: &Matrix) -> BFSSolver {
        BFSSolver { matrix, path: None }
    }

    pub fn solve<F>(&mut self, mut traverse: F) where F: FnMut(&Matrix, &Cell) {
        let mut trace: HashMap<Cell, Cell> = HashMap::new();
        let mut visited: HashSet<Cell> = HashSet::new();
        let mut que: VecDeque<Cell> = VecDeque::new();
        que.push_back(self.matrix.start);
        visited.insert(self.matrix.start);
        while que.len() > 0 {
            let cell = que.pop_front().expect("queue is empty");
            traverse(self.matrix, &cell);
            if cell == self.matrix.finish {
                break;
            }
            for dir in Direction::get_dirs().into_iter() {
                let new_cell = cell.move_dir(dir);
                if self.matrix.is_free_cell(new_cell) && !visited.contains(&new_cell) {
                    visited.insert(new_cell);
                    que.push_back(new_cell);
                    trace.insert(new_cell, new_cell.move_dir(dir.reverse()));
                }
            }
        }
        if trace.get(&self.matrix.finish).is_some() {
            let mut path: Vec<Cell> = Vec::new();
            let mut cell_opt = Some(&self.matrix.finish);
            while let Some(cur_cell) = cell_opt {
                path.push(cur_cell.clone());
                cell_opt = trace.get(&cur_cell);
            }
            path.reverse();
            self.path = Some(path);
            println!("FOUND PATH {:?}", &self.path)
        } else {
            println!("NOT FOUND PATH")
        }
    }
}
