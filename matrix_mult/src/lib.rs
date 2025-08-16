use std::ops::{Mul ,  Add};
use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T>
where
    T: Scalar<Item = T> + Clone + Mul<Output = T>  ,
{
    pub fn new() -> Self {
        Matrix(vec![])
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Matrix(vec![vec![T::zero(); cols]; rows])
    }

    pub fn identity(size: usize) -> Self {
        let mut m = Self::zero(size, size);
        for i in 0..size {
            m.0[i][i] = T::one();
        }
        m
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn row(&self, index: usize) -> Vec<T> {
        self.0.get(index).cloned().expect("")
    }

    pub fn col(&self, index: usize) -> Vec<T> {
        if index >= self.number_of_cols() {
            return vec![];
        }
        self.0.iter().map(|row| row[index].clone()).collect()
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Scalar<Item = T> + Add<Output = T> + Clone + Mul<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let rows = self.number_of_rows();
        let cols = other.number_of_cols();
        let mut result = Matrix::zero(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                let mut sum = T::zero();
                for k in 0..self.number_of_cols() {
                    sum = sum + self.0[i][k].clone() * other.0[k][j].clone();
                }
                result.0[i][j] = sum;
            }
        }

        Some(result)
    }
}
