#![allow(dead_code)]
use std::fmt::{Debug, Formatter, Result};

#[derive(Debug)]
pub struct Matrix {
    values: Vec<Vec<Elem>>
}

// #[derive(Debug)]
pub struct Elem {
    value: char,
    row: usize,
    col: usize
}

impl Debug for Elem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Elem {
    fn new (value: char, row: usize, col: usize) -> Elem {
        Elem {
            value,
            row,
            col
        }
    }

    pub fn is_side (&self, matrix: &Matrix) -> bool {
        if self.row == 0 || self.row == matrix.values.len() - 1 
        || self.col == 0 || self.col == matrix.values[0].len() - 1 {
            true
        }
        else {
            false
        }
    }

    pub fn is_edge (&self, matrix: &Matrix) -> bool {
        if (self.row == 0 && self.col == 0)
        || (self.row == 0 && self.col == matrix.values[0].len() - 1)
        || (self.row == matrix.values.len() - 1 && self.col == 0)
        || (self.row == matrix.values.len() - 1 && self.col == matrix.values[0].len() - 1) {
            true
        }
        else {
            false
        }
    }

    pub fn get_neighbors (&self, matrix: &Matrix) -> Vec<Elem> {
        let mut elems: Vec<Elem> = vec![];
        let max_i = matrix.values.len() - 1;
        let max_j = matrix.values[0].len() - 1;
        if !self.is_edge(matrix)
        && !self.is_side(matrix) {
            println!("HI!");
            elems.push(Elem::new(matrix.get_element(self.row, self.col + 1).value, self.row, self.col + 1));
            elems.push(Elem::new(matrix.get_element(self.row, self.col - 1).value, self.row, self.col - 1));
            elems.push(Elem::new(matrix.get_element(self.row + 1, self.col).value, self.row + 1, self.col));
            elems.push(Elem::new(matrix.get_element(self.row + 1, self.col - 1).value, self.row + 1, self.col - 1));
            elems.push(Elem::new(matrix.get_element(self.row + 1, self.col + 1).value, self.row + 1, self.col + 1));
            elems.push(Elem::new(matrix.get_element(self.row - 1, self.col).value, self.row - 1, self.col));
            elems.push(Elem::new(matrix.get_element(self.row - 1, self.col + 1).value, self.row - 1, self.col + 1));
            elems.push(Elem::new(matrix.get_element(self.row - 1, self.col - 1).value, self.row - 1, self.col - 1));
        }
        else if self.is_edge(matrix) {
            println!("ROFL!");
            match (self.row, self.col) {
                (0, 0) => {
                    elems.push(Elem::new(matrix.get_element(self.row, self.col + 1).value, self.row, self.col + 1));
                    elems.push(Elem::new(matrix.get_element(self.row + 1, self.col + 1).value, self.row + 1, self.col + 1));
                    elems.push(Elem::new(matrix.get_element(self.row + 1, self.col).value, self.row + 1, self.col));
                },
                (0, max_j) => {
                    elems.push(Elem::new(matrix.get_element(self.row, self.col - 1).value, self.row, self.col - 1));
                    elems.push(Elem::new(matrix.get_element(self.row + 1, self.col - 1).value, self.row + 1, self.col - 1));
                    elems.push(Elem::new(matrix.get_element(self.row + 1, self.col).value, self.row + 1, self.col));
                },
                (max_i, 0) => {
                    elems.push(Elem::new(matrix.get_element(self.row, self.col + 1).value, self.row, self.col + 1));
                    elems.push(Elem::new(matrix.get_element(self.row - 1, self.col + 1).value, self.row - 1, self.col + 1));
                    elems.push(Elem::new(matrix.get_element(self.row - 1, self.col).value, self.row - 1, self.col));
                },
                (max_i, max_j) => {
                    elems.push(Elem::new(matrix.get_element(self.row, self.col - 1).value, self.row, self.col - 1));
                    elems.push(Elem::new(matrix.get_element(self.row - 1, self.col - 1).value, self.row - 1, self.col - 1));
                    elems.push(Elem::new(matrix.get_element(self.row - 1, self.col).value, self.row - 1, self.col));
                }
            }
        }
        else if self.is_side(matrix) {
            if self.row == 0 && (self.col != 0 || self.col != max_j) {
                elems.push(Elem::new(matrix.get_element(self.row, self.col - 1).value, self.row, self.col - 1));
                elems.push(Elem::new(matrix.get_element(self.row, self.col + 1).value, self.row, self.col + 1));
                elems.push(Elem::new(matrix.get_element(self.row + 1, self.col + 1).value, self.row + 1, self.col + 1));
                elems.push(Elem::new(matrix.get_element(self.row + 1, self.col).value, self.row + 1, self.col));
                elems.push(Elem::new(matrix.get_element(self.row + 1, self.col - 1).value, self.row + 1, self.col - 1));
            }
            else if self.col == 0 && (self.row != 0 || self.row != max_i) {
                elems.push(Elem::new(matrix.get_element(self.row + 1, self.col).value, self.row + 1, self.col));
                elems.push(Elem::new(matrix.get_element(self.row - 1, self.col).value, self.row - 1, self.col));
                elems.push(Elem::new(matrix.get_element(self.row + 1, self.col + 1).value, self.row + 1, self.col + 1));
                elems.push(Elem::new(matrix.get_element(self.row, self.col + 1).value, self.row, self.col + 1));
                elems.push(Elem::new(matrix.get_element(self.row - 1, self.col - 1).value, self.row - 1, self.col - 1));
            }
            else if self.col == max_j && (self.row != 0 || self.row != max_i) {
                elems.push(Elem::new(matrix.get_element(self.row + 1, self.col).value, self.row + 1, self.col));
                elems.push(Elem::new(matrix.get_element(self.row - 1, self.col).value, self.row - 1, self.col));
                elems.push(Elem::new(matrix.get_element(self.row + 1, self.col - 1).value, self.row + 1, self.col - 1));
                elems.push(Elem::new(matrix.get_element(self.row, self.col - 1).value, self.row, self.col - 1));
                elems.push(Elem::new(matrix.get_element(self.row - 1, self.col - 1).value, self.row - 1, self.col - 1));
            }
            else if self.row == max_i && (self.col != 0 || self.col != max_j) {
                elems.push(Elem::new(matrix.get_element(self.row, self.col - 1).value, self.row, self.col - 1));
                elems.push(Elem::new(matrix.get_element(self.row, self.col + 1).value, self.row, self.col + 1));
                elems.push(Elem::new(matrix.get_element(self.row - 1, self.col + 1).value, self.row - 1, self.col + 1));
                elems.push(Elem::new(matrix.get_element(self.row - 1, self.col).value, self.row - 1, self.col));
                elems.push(Elem::new(matrix.get_element(self.row - 1, self.col - 1).value, self.row - 1, self.col - 1));
            }
        }
        elems
    }
}

impl Matrix {
    pub fn new (data: &String) -> Matrix {
        let mut values: Vec<Vec<Elem>> = vec![];
        for (i, line) in data.lines().enumerate() {
            let mut cols: Vec<Elem> = vec![];
            for (j, ch) in line.chars().enumerate() {
                cols.push(Elem::new(ch, i, j));
            }
            values.push(cols);
        }
        Matrix {
            values
        }
    }

    pub fn traverse (&self) -> Vec<Vec<u8>> {
        let mut transformed: Vec<Vec<u8>> = vec![];
        for i in 0..self.values.len() {
            let mut temp: Vec<u8> = vec![];
            for j in 0..self.values[0].len() {
                let ele = &self.values[i][j];
                if ele.is_edge(&self) {
                    temp.push(1);
                }
                else if ele.is_side(&self) {
                    temp.push(0);
                }
                else {
                    temp.push(2);
                }
            }
            transformed.push(temp);
        }
        transformed
    }

    pub fn get_element (&self, i: usize, j: usize) -> &Elem {
        &self.values[i][j]
    }
}