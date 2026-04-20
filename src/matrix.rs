use std::{ops::{Index, IndexMut}, vec};

pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}
impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix { data: vec![0.0; rows*cols], rows: rows, cols: cols }
    }
    pub fn new1(rows: usize, cols: usize) -> Self {
        Matrix { data: vec![1.0; rows*cols], rows: rows, cols: cols }
    }
}
impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.data[i*self.cols + j]
    }
}
impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        &mut self.data[i*self.cols + j]
    }
}