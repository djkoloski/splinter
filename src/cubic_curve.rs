use crate::{
    InnerProductSpace,
    QuadraticCurve,
    Scalar,
};

pub struct CubicCurve<T: InnerProductSpace> {
    x0: T,
    x1: T,
    x2: T,
    x3: T,
}

impl<T: InnerProductSpace> CubicCurve<T> {
    pub fn new(x0: T, x1: T, x2: T, x3: T) -> CubicCurve<T> {
        CubicCurve {
            x0,
            x1,
            x2,
            x3,
        }
    }

    pub fn x0(&self) -> T {
        self.x0
    }

    pub fn x1(&self) -> T {
        self.x1
    }

    pub fn x2(&self) -> T {
        self.x2
    }

    pub fn x3(&self) -> T {
        self.x3
    }

    pub fn evaluate(&self, t: T::Scalar) -> T {
        self.x0 + (self.x1 + (self.x2 + self.x3 * t) * t) * t
    }

    pub fn evaluate_first_derivative(&self, t: T::Scalar) -> T {
        self.x1 + self.x2 * t * T::Scalar::from_i32(2) + self.x3 * t * t * T::Scalar::from_i32(3)
    }

    pub fn evaluate_second_derivative(&self, t: T::Scalar) -> T {
        self.x2 * T::Scalar::from_i32(2) + self.x3 * t * T::Scalar::from_i32(6)
    }

    pub fn evaluate_third_derivative(&self) -> T {
        self.x3 * T::Scalar::from_i32(6)
    }

    pub fn from_bezier(b0: T, b1: T, b2: T, b3: T) -> CubicCurve<T> {
        CubicCurve {
            x0: b0,
            x1: b0 * T::Scalar::from_i32(-3) + b1 * T::Scalar::from_i32(3),
            x2: b0 * T::Scalar::from_i32(3) + b1 * T::Scalar::from_i32(-6) + b2 * T::Scalar::from_i32(3),
            x3: b0 * T::Scalar::from_i32(-1) + b1 * T::Scalar::from_i32(3) + b2 * T::Scalar::from_i32(-3) + b3,
        }
    }

    pub fn to_bezier(&self) -> (T, T, T, T) {
        (
            self.x0,
            self.x0 + self.x1 / T::Scalar::from_i32(3),
            self.x0 + self.x1 * T::Scalar::from_i32(2) / T::Scalar::from_i32(3) + self.x2 / T::Scalar::from_i32(3),
            self.x0 + self.x1 + self.x2 + self.x3,
        )
    }

    pub fn subdivide(&self, u: T::Scalar) -> (CubicCurve<T>, CubicCurve<T>) {
        let x1u = self.x1 * u;
        let x2u = self.x2 * u;
        let x2u2 = x2u * u;
        let x3u = self.x3 * u;
        let x3u2 = x3u * u;
        let x3u3 = x3u2 * u;
        let w = T::Scalar::from_i32(1) - u;
        let w2 = w * w;

        (
            CubicCurve::new(
                self.x0,
                x1u,
                x2u2,
                x3u3,
            ),
            CubicCurve::new(
                self.x0 + x1u + x2u2 + x3u3,
                (self.x1 + x2u * T::Scalar::from_i32(2) + x3u2 * T::Scalar::from_i32(3)) * w,
                (self.x2 + x3u * T::Scalar::from_i32(3)) * w2,
                self.x3 * w2 * w,
            )
        )
    }

    pub fn error_minimizing_quadratic_approximation(&self) -> QuadraticCurve<T> {
        QuadraticCurve::new(
            self.x0 + self.x3 / T::Scalar::from_i32(32),
            self.x1 - self.x3 * T::Scalar::from_i32(9) / T::Scalar::from_i32(16),
            self.x2 + self.x3 * T::Scalar::from_i32(3) / T::Scalar::from_i32(2),
        )
    }

    pub fn continuity_preseving_quadratic_approximation(&self) -> QuadraticCurve<T> {
        QuadraticCurve::new(
            self.x0,
            self.x1 - self.x3 / T::Scalar::from_i32(2),
            self.x2 + self.x3 * T::Scalar::from_i32(3) / T::Scalar::from_i32(2),
        )
    }
}

#[cfg(test)]
mod tests {
    mod cubic_curve {
        use crate::CubicCurve;
        use approx::assert_ulps_eq;

        #[test]
        fn evaluate() {
            let curve = CubicCurve::new(1f32, 1f32, 1f32, 1f32);
            assert_ulps_eq!(curve.evaluate_third_derivative(), 6f32);

            assert_ulps_eq!(curve.evaluate(0f32), 1f32);
            assert_ulps_eq!(curve.evaluate_first_derivative(0f32), 1f32);
            assert_ulps_eq!(curve.evaluate_second_derivative(0f32), 2f32);

            assert_ulps_eq!(curve.evaluate(0.5f32), 1.875f32);
            assert_ulps_eq!(curve.evaluate_first_derivative(0.5f32), 2.75f32);
            assert_ulps_eq!(curve.evaluate_second_derivative(0.5f32), 5f32);

            assert_ulps_eq!(curve.evaluate(1f32), 4f32);
            assert_ulps_eq!(curve.evaluate_first_derivative(1f32), 6f32);
            assert_ulps_eq!(curve.evaluate_second_derivative(1f32), 8f32);
        }

        #[test]
        fn bezier() {
            let test_cases = [
                (1f32, 1f32, 1f32, 1f32),
                (2f32, 0f32, 0f32, 1f32),
                (-1f32, 1f32, 0.5f32, -0.25f32),
                (5f32, 4f32, 3f32, 2f32),
                (6f32, -7f32, -9f32, -1000f32),
            ];
            for test_case in &test_cases {
                let curve = CubicCurve::from_bezier(test_case.0, test_case.1, test_case.2, test_case.3);
                let bezier = curve.to_bezier();
                assert_ulps_eq!(bezier.0, test_case.0);
                assert_ulps_eq!(bezier.1, test_case.1);
                assert_ulps_eq!(bezier.2, test_case.2);
                assert_ulps_eq!(bezier.3, test_case.3);
            }
        }

        #[test]
        fn subdivide() {
            let curve = CubicCurve::new(1f32, 1f32, 1f32, 1f32);
            let (left, right) = curve.subdivide(0.25f32);

            assert_ulps_eq!(left.x0(), 1f32);
            assert_ulps_eq!(left.x1(), 0.25f32);
            assert_ulps_eq!(left.x2(), 0.0625f32);
            assert_ulps_eq!(left.x3(), 0.015625f32);

            assert_ulps_eq!(right.x0(), 1.328125f32);
            assert_ulps_eq!(right.x1(), 1.265625f32);
            assert_ulps_eq!(right.x2(), 0.984375f32);
            assert_ulps_eq!(right.x3(), 0.421875f32);
        }

        #[test]
        fn error_minimizing_quadratic_approximation() {
            let curve = CubicCurve::new(1f32, 1f32, 1f32, 1f32);
            let quadratic = curve.error_minimizing_quadratic_approximation();

            assert_ulps_eq!(curve.evaluate(0.5f32), quadratic.evaluate(0.5f32));
            let left_zero = 0.25f32 * (2f32 - 3f32.sqrt());
            assert_ulps_eq!(curve.evaluate(left_zero), quadratic.evaluate(left_zero));
            let right_zero = 0.25f32 * (2f32 + 3f32.sqrt());
            assert_ulps_eq!(curve.evaluate(right_zero), quadratic.evaluate(right_zero));

            assert_ulps_eq!((curve.evaluate(0f32) - quadratic.evaluate(0f32)).abs(), 0.03125f32);
            assert_ulps_eq!((curve.evaluate(0.25f32) - quadratic.evaluate(0.25f32)).abs(), 0.03125f32);
            assert_ulps_eq!((curve.evaluate(0.75f32) - quadratic.evaluate(0.75f32)).abs(), 0.03125f32);
            assert_ulps_eq!((curve.evaluate(1f32) - quadratic.evaluate(1f32)).abs(), 0.03125f32);
        }

        #[test]
        fn continuity_preseving_quadratic_approximation() {
            let curve = CubicCurve::new(1f32, 1f32, 1f32, 1f32);
            let quadratic = curve.continuity_preseving_quadratic_approximation();

            assert_ulps_eq!(curve.evaluate(0f32), quadratic.evaluate(0f32));
            assert_ulps_eq!(curve.evaluate(0.5f32), quadratic.evaluate(0.5f32));
            assert_ulps_eq!(curve.evaluate(1f32), quadratic.evaluate(1f32));

            let max_error = 1f32 / 12f32 / 3f32.sqrt();
            let left_max = 0.5f32 - 1f32 / 2f32 / 3f32.sqrt();
            let right_max = 0.5f32 + 1f32 / 2f32 / 3f32.sqrt();
            assert_ulps_eq!((curve.evaluate(left_max) - quadratic.evaluate(left_max)).abs(), max_error);
            assert_ulps_eq!((curve.evaluate(right_max) - quadratic.evaluate(right_max)).abs(), max_error);
        }
    }
}