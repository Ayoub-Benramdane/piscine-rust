use lalgebra_scalar::Scalar;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T> + std::clone::Clone> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut vec = Vec::with_capacity(row);
        for _ in 0..row {
            vec.push(vec![T::zero(); col]);
        }
        Matrix(vec)
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut vec = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = vec![T::zero(); n];
            row[i] = T::one();
            vec.push(row);
        }
        Matrix(vec)
	}
}

impl<T: Scalar + Add<Output = T> + Copy> Add for Matrix<T>  {
    type Output = Option<Matrix<T>>;
    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        let mut a = vec![];
        for (i, j) in self.0.iter().zip(other.0.iter()) {
            if i.len() != j.len() {
                return None;
            }
            let mut ind = vec![];
            for (val_self, val_other) in i.iter().zip(j.iter()) {
                ind.push(*val_self + *val_other);
            }
            a.push(ind);
        }
        Some(Matrix(a))
    }
}

impl<T: Scalar + Sub<Output = T> + Copy> Sub for Matrix<T>  {
    type Output = Option<Matrix<T>>;
    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        let mut a = vec![];
        for (i, j) in self.0.iter().zip(other.0.iter()) {
            if i.len() != j.len() {
                return None;
            }
            let mut ind = vec![];
            for (val_self, val_other) in i.iter().zip(j.iter()) {
                ind.push(*val_self - *val_other);
            }
            a.push(ind);
        }
        Some(Matrix(a))
    }
}