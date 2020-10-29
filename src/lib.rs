mod vector2;
mod cubic_curve;
mod linear_curve;
mod quadratic_curve;
mod roots;

pub use self::cubic_curve::CubicCurve;
pub use self::linear_curve::LinearCurve;
pub use self::quadratic_curve::QuadraticCurve;

use std::{
    cmp::PartialOrd,
    f32,
    f64,
    fmt::Debug,
    ops::{
        Add,
        AddAssign,
        Div,
        DivAssign,
        Mul,
        MulAssign,
        Neg,
        Sub,
        SubAssign,
    },
};

pub trait Scalar: Add<Output = Self> + AddAssign + Clone + Copy + Debug + Div<Output = Self> + DivAssign + Mul<Output = Self> + MulAssign + Neg<Output = Self> + PartialEq + PartialOrd + Sub<Output = Self> + SubAssign {
    fn abs(self) -> Self;
    fn acos(self) -> Self;
    fn cbrt(self) -> Self;
    fn cos(self) -> Self;
    fn from_i32(value: i32) -> Self;
    fn ln(self) -> Self;
    fn pi() -> Self;
    fn sqrt(self) -> Self;
}

pub trait VectorSpace: Add<Output = Self> + AddAssign + Clone + Copy + Div<<Self as VectorSpace>::Scalar, Output = Self> + DivAssign<<Self as VectorSpace>::Scalar> + Mul<<Self as VectorSpace>::Scalar, Output = Self> + MulAssign<<Self as VectorSpace>::Scalar> + Neg + Sub<Output = Self> + SubAssign {
    type Scalar: Scalar;

    fn zero() -> Self;
}

pub trait InnerProductSpace: VectorSpace {
    fn inner_product(self, other: Self) -> Self::Scalar;
}

impl Scalar for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn cbrt(self) -> Self {
        self.cbrt()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn from_i32(value: i32) -> Self {
        value as f32
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn pi() -> Self {
        f32::consts::PI
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl VectorSpace for f32 {
    type Scalar = f32;

    fn zero() -> Self {
        0f32
    }
}

impl InnerProductSpace for f32 {
    fn inner_product(self, other: Self) -> Self::Scalar {
        self * other
    }
}

impl Scalar for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn cbrt(self) -> Self {
        self.cbrt()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn from_i32(value: i32) -> Self {
        value as f64
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn pi() -> Self {
        f64::consts::PI
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl VectorSpace for f64 {
    type Scalar = f64;

    fn zero() -> Self {
        0f64
    }
}

impl InnerProductSpace for f64 {
    fn inner_product(self, other: Self) -> Self::Scalar {
        self * other
    }
}