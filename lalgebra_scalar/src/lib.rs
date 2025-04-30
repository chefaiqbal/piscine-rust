use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized + Copy
{
    fn zero() -> Self;
    fn one() -> Self;
}


macro_rules! impl_scalar_int {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                fn zero() -> Self { 0 }
                fn one() -> Self { 1 }
            }
        )*
    }
}


macro_rules! impl_scalar_float {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                fn zero() -> Self { 0.0 }
                fn one() -> Self { 1.0 }
            }
        )*
    }
}


impl_scalar_int!(u32, u64, i32, i64);
impl_scalar_float!(f32, f64);