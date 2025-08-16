use lalgebra_scalar::Scalar;

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