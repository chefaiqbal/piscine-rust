use std::ops::Mul;
use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
   
        self.0.first().map_or(0, |row| row.len())
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0.get(n).cloned().unwrap_or_default()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter()
            .filter_map(|row| row.get(n).cloned())
            .collect()
    }
}

impl<T: Scalar + Clone> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
     
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

      
        let rows = self.number_of_rows();
        let cols = other.number_of_cols();
        
   
        let other_cols: Vec<Vec<T>> = (0..cols)
            .map(|j| other.col(j))
            .collect();

        let result = (0..rows)
            .map(|i| {
                let row_i = self.row(i);
                (0..cols)
                    .map(|j| {
                
                        row_i.iter()
                            .zip(other_cols[j].iter())
                            .fold(T::zero(), |acc, (a, b)| acc + (*a).clone() * (*b).clone())
                    })
                    .collect()
            })
            .collect();

        Some(Matrix(result))
    }
}
/*
Instructions

Implement the methods:

    number_of_cols: which returns the number of columns in the matrix.
    number_of_rows: which returns the number of rows in the matrix.
    row: which returns the nth row in the matrix.
    col: which returns the nth column in the matrix.

Define the matrix multiplication by implementing the std::ops::Mul for the type matrix
Expected Functions

impl Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
	}

	pub fn number_of_rows(&self) -> usize {
	}

	pub fn row(&self, n: usize) -> Vec<T> {
	}

	pub fn col(&self, n: usize) -> Vec<T> {
	}
}

impl Mul for Matrix<T> {
}

Usage

Here is a program to test your function.

use matrix_mult::*;

fn main() {
	let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
	println!("{:?}", matrix.col(0));
	println!("{:?}", matrix.row(1));

	let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
	let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
	let mult = matrix_1.clone() * matrix_2.clone();
	println!("{:?}", mult);
	println!("{:?}", matrix_1.number_of_cols());
	println!("{:?}", matrix_2.number_of_rows());
}

And its output

$ cargo run
[3, 8]
[8, 0]
Some(Matrix([[1, 0], [0, 0]]))
2
2
$

*/