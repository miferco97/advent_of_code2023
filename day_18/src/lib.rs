use std::fmt;
use std::ops::{Index, IndexMut};

#[derive (Clone)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: Clone + fmt::Display> Matrix<T> {
    pub fn new(rows: usize, cols: usize, value: T) -> Matrix<T> {
        let data = vec![value; (rows * cols) as usize];
        Matrix { data, rows, cols }
    }
    pub fn at(&self, i: usize, j: usize) -> &T {
        let index = i * self.cols + j;
        &self.data[index as usize]
    }
    pub fn at_mut(&mut self, i: usize, j: usize) -> &mut T {
        let index = i * self.cols + j;
        &mut self.data[index as usize]
    }
    pub fn col(&self, j: usize) -> Vec<&T> {
        let mut col: Vec<&T> = Vec::new();
        for i in 0..self.rows {
            col.push(self.at(i, j));
        }
        col
    }
    pub fn row(&self, i: usize) -> Vec<&T> {
        let mut row: Vec<&T> = Vec::new();
        for j in 0..self.cols {
            row.push(self.at(i, j));
        }
        row
    }

    pub fn substitute_row(&mut self, i: usize, row: Vec<T>) {
        for j in 0..self.cols {
            self.data[(i * self.cols + j) as usize] = row[j as usize].clone();
        }
    }
    pub fn substitute_col(&mut self, j: usize, col: Vec<T>) {
        for i in 0..self.rows {
            self.data[(i * self.cols + j) as usize] = col[i as usize].clone();
        }
    }
}

impl Matrix<char> {
    pub fn new_from_str(str: &str) -> Matrix<char> {
        let mut n_rows: usize = 0;
        let mut n_cols: usize = 0;
        let mut data: Vec<char> = Vec::new();

        for elems in str.lines() {
            n_rows += 1;
            n_cols = elems.len() as usize;
            for elem in elems.chars() {
                data.push(elem);
            }
        }

        Matrix {
            data,
            rows: n_rows,
            cols: n_cols,
        }
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", *self.at(i, j))?;
            }
            write!(f, "\n")?;
        }
        std::fmt::Result::Ok(())
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
