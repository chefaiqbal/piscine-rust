use std::ops::{Add, Sub};
use lalgebra_scalar::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + std::clone::Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        Matrix(
            (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| if i == j { T::one() } else { T::zero() })
                        .collect()
                })
                .collect()
        )
    }
}

impl<T: Scalar + Copy> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        // Better dimension check that handles empty matrices
        let valid_dimensions = self.0.len() == other.0.len() && 
            self.0.iter().zip(other.0.iter())
                .all(|(row1, row2)| row1.len() == row2.len());
        
        if !valid_dimensions {
            return None;
        }

        Some(Matrix(
            self.0.into_iter()
                .zip(other.0.into_iter())
                .map(|(row1, row2)| {
                    row1.into_iter()
                        .zip(row2.into_iter())
                        .map(|(val1, val2)| val1 - val2)
                        .collect()
                })
                .collect()
        ))
    }
}

impl<T: Scalar + Copy> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        // Better dimension check that handles empty matrices
        let valid_dimensions = self.0.len() == other.0.len() && 
            self.0.iter().zip(other.0.iter())
                .all(|(row1, row2)| row1.len() == row2.len());
        
        if !valid_dimensions {
            return None;
        }

        Some(Matrix(
            self.0.into_iter()
                .zip(other.0.into_iter())
                .map(|(row1, row2)| {
                    row1.into_iter()
                        .zip(row2.into_iter())
                        .map(|(val1, val2)| val1 + val2)
                        .collect()
                })
                .collect()
        ))
    }
}

/*
Instructions

In this exercise, you will define some basic matrix operations, Implement traits for Add and Sub

Remember that two matrices can only be added or subtracted if they have the same dimensions. Therefore, you must handle the possibility of failure by returning an Option<T>.

You will be reusing your Matrix and Scalar structures defined in the matrix and lalgebra-scalar exercises.
Expected Function

impl Add for Matrix {

}

impl Sub for Matrix {

}

Usage

Here is a program to test your function

// Importing Matrix by defining it as a dependency in Cargo.toml
use matrix_ops::*;

fn main() {
	let matrix = Matrix(vec![vec![8, 1], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![2, 5]]);
	let matrix_2 = Matrix(vec![vec![3, 1], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);
}

And its output

$ cargo run
Some(Matrix([[9, 2], [10, 2]]))
Some(Matrix([[-2, 2], [1, 4]]))
None
None
$

*/