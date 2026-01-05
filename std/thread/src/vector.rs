use std::ops::{Add, AddAssign, Deref, Mul};

use anyhow::Result;
pub struct Vector<T> {
    pub data: Vec<T>,
}
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}
// pretend this is a heavy operation,CPU intensive
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Add<Output = T> + AddAssign + Mul<Output = T> + Default,
{
    if a.len() != b.len() {
        panic!("a and b must have the same length");
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}
