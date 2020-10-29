use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};
use crate::{
    Scalar,
    VectorSpace,
    InnerProductSpace,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vector2<T: Scalar> {
    pub x: T,
    pub y: T,
}

impl<T: Scalar> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Scalar> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Scalar> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Vector2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T: Scalar> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
    }
}

impl<T: Scalar> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: Scalar> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T: Scalar> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Scalar> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Scalar> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Scalar> VectorSpace for Vector2<T> {
    type Scalar = T;

    fn zero() -> Self {
        Vector2::new(Scalar::from_i32(0), Scalar::from_i32(0))
    }
}

impl<T: Scalar> InnerProductSpace for Vector2<T> {
    fn inner_product(self, other: Self) -> Self::Scalar {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Scalar> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {
            x,
            y,
        }
    }
}