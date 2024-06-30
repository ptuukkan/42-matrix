use std::{array, ops};

use crate::vector::Vector;

impl<K, const N: usize> ops::Add for Vector<K, N>
where
    K: ops::Add<Output = K> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl<K, const N: usize> ops::Sub for Vector<K, N>
where
    K: ops::Sub<Output = K> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl<K, const N: usize> ops::Mul<K> for Vector<K, N>
where
    K: ops::Mul<Output = K> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: K) -> Self::Output {
        Self(array::from_fn(|i| self.0[i] * rhs))
    }
}

#[test]
fn test_vector_add_2() {
    let u = Vector([2.0, 3.0]);
    let v = Vector([5.0, 7.0]);
    assert_eq!(u + v, Vector([7.0, 10.0]))
}

#[test]
fn test_vector_add_3() {
    let u = Vector::from([2.0, 3.0, -5.0]);
    let v = Vector::from([5.0, 7.0, 2.0]);
    assert_eq!(u + v, Vector([7.0, 10.0, -3.0]));
}

#[test]
fn test_vector_sub_2() {
    let u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);
    assert_eq!(u - v, Vector([-3.0, -4.0]));
}

#[test]
fn test_vector_sub_3() {
    let u = Vector::from([2.0, 3.0, -5.0]);
    let v = Vector::from([5.0, 7.0, 2.0]);
    assert_eq!(u - v, Vector([-3.0, -4.0, -7.0]));
}

#[test]
fn test_vector_mul_2() {
    let u = Vector::from([2.0, 3.0]);
    assert_eq!(u * 2.0, Vector([4.0, 6.0]));
}

#[test]
fn test_vector_mul_3() {
    let u = Vector::from([2.0, 3.0, -5.0]);
    assert_eq!(u * -1.5, Vector([-3.0, -4.5, 7.5]));
}
