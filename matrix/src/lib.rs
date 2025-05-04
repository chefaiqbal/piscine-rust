use lalgebra_scalar::*;
use std::ops::{Index, IndexMut};
use std::iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Clone> Matrix<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self(iter::repeat(iter::repeat(T::zero()).take(cols).collect())
            .take(rows)
            .collect())
    }

    pub fn identity(n: usize) -> Self {
        Self(
            (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| if i == j { T::one() } else { T::zero() })
                        .collect()
                })
                .collect(),
        )
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Scalar + Clone> Default for Matrix<T> {
    fn default() -> Self {
        Self(vec![vec![T::zero()]])
    }
}
  
/*
Instructions

Define a data structure to represent a matrix of any size and implement some basic operations.

We will consider a matrix as a rectangular arrangements of scalars. You can represent this as a 2 dimensional vector`. You will use the definition of scalars from the lalgebra-scalar exercise.

Implement the following associated functions:

    new: which returns a matrix of size 1 x 1.
    identity: which returns the identity matrix of size n.
    zero: which returns a matrix of size row x col with all the positions filled by zeros.

Expected Functions and Structure

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
	}

	pub fn identity(n: usize) -> Matrix<T> {
	}
}

Usage

Here is a program to test your function.

use matrix::*;

fn main() {
	let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
	println!("{:?}", m);
	println!("{:?}", Matrix::<i32>::identity(4));
	println!("{:?}", Matrix::<f64>::zero(3, 4));
}

And its output:

$ cargo run
Matrix([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]])
Matrix([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]])
Matrix([[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]])
$

Notions

    Traits

*/