use crate::{
    InnerProductSpace,
    QuadraticCurve,
    Scalar,
};

pub struct LinearCurve<T: InnerProductSpace> {
    x0: T,
    x1: T,
}

impl<T: InnerProductSpace> LinearCurve<T> {
    pub fn new(x0: T, x1: T) -> LinearCurve<T> {
        LinearCurve {
            x0,
            x1,
        }
    }

    pub fn x0(&self) -> T {
        self.x0
    }

    pub fn x1(&self) -> T {
        self.x1
    }

    pub fn evaluate(&self, t: T::Scalar) -> T {
        self.x0 + self.x1 * t
    }

    pub fn evaluate_derivative(&self) -> T {
        self.x1
    }

    pub fn from_bezier(b0: T, b1: T) -> LinearCurve<T> {
        LinearCurve {
            x0: b0,
            x1: b1 - b0
        }
    }

    pub fn to_bezier(&self) -> (T, T) {
        (
            self.x0,
            self.x0 + self.x1,
        )
    }

    pub fn subdivide(&self, u: T::Scalar) -> (LinearCurve<T>, LinearCurve<T>) {
        let x1u = self.x1 * u;

        (
            LinearCurve::new(
                self.x0,
                x1u,
            ),
            LinearCurve::new(
                self.x0 + x1u,
                x1u,
            )
        )
    }

    pub fn to_quadratic(&self) -> QuadraticCurve<T> {
        QuadraticCurve::new(
            self.x0,
            self.x1,
            T::zero(),
        )
    }

    pub fn speed(&self) -> T::Scalar {
        self.x1.inner_product(self.x1).sqrt()
    }

    pub fn length(&self, u: T::Scalar) -> T::Scalar {
        u * self.speed()
    }

    pub fn parameterize(&self, v: T::Scalar) -> Option<T::Scalar> {
        let speed = self.speed();
        if speed == T::Scalar::from_i32(0) {
            None
        } else {
            Some(v / self.speed())
        }
    }

    pub fn nearest_point(&self, q: T) -> T::Scalar {
        self.x1.inner_product(q - self.x0)
    }
}