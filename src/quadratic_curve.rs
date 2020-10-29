use crate::{
    CubicCurve,
    InnerProductSpace,
    LinearCurve,
    Scalar,
    roots::solve_cubic,
};

pub struct QuadraticCurve<T: InnerProductSpace> {
    x0: T,
    x1: T,
    x2: T,
}

impl<T: InnerProductSpace> QuadraticCurve<T> {
    pub fn new(x0: T, x1: T, x2: T) -> QuadraticCurve<T> {
        QuadraticCurve {
            x0,
            x1,
            x2,
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

    pub fn evaluate(&self, t: T::Scalar) -> T {
        self.x0 + (self.x1 + self.x2 * t) * t
    }

    pub fn evaluate_first_derivative(&self, t: T::Scalar) -> T {
        self.x1 + self.x2 * t * T::Scalar::from_i32(2)
    }

    pub fn evaluate_second_derivative(&self) -> T {
        self.x2 * T::Scalar::from_i32(2)
    }

    pub fn from_bezier(b0: T, b1: T, b2: T) -> QuadraticCurve<T> {
        QuadraticCurve {
            x0: b0,
            x1: b0 * T::Scalar::from_i32(-2) + b1 * T::Scalar::from_i32(2),
            x2: b0 + b1 * T::Scalar::from_i32(-2) + b2,
        }
    }

    pub fn to_bezier(&self) -> (T, T, T) {
        (
            self.x0,
            self.x0 + self.x1 / T::Scalar::from_i32(2),
            self.x0 + self.x1 + self.x2,
        )
    }

    pub fn subdivide(&self, u: T::Scalar) -> (QuadraticCurve<T>, QuadraticCurve<T>) {
        let x1u = self.x1 * u;
        let x2u = self.x2 * u;
        let x2u2 = x2u * u;
        let w = T::Scalar::from_i32(1) - u;

        (
            QuadraticCurve::new(
                self.x0,
                x1u,
                x2u2,
            ),
            QuadraticCurve::new(
                self.x0 + x1u + x2u2,
                (self.x1 + x2u * T::Scalar::from_i32(2)) * w,
                self.x2 * w * w,
            )
        )
    }

    pub fn to_cubic(&self) -> CubicCurve<T> {
        CubicCurve::new(
            self.x0,
            self.x1,
            self.x2,
            T::zero(),
        )
    }

    fn speed_coefficients(&self) -> (T::Scalar, T::Scalar, T::Scalar) {
        let a = self.x2.inner_product(self.x2) * T::Scalar::from_i32(4);
        let b = self.x1.inner_product(self.x2) * T::Scalar::from_i32(4);
        let c = self.x1.inner_product(self.x1);
        (c, b, a)
    }

    fn speed_from_coefficients(&self, u: T::Scalar, coefficients: (T::Scalar, T::Scalar, T::Scalar)) -> T::Scalar {
        let (c, b, a) = coefficients;
        (c + u * (b + a * u)).sqrt()
    }

    pub fn speed(&self, u: T::Scalar) -> T::Scalar {
        self.speed_from_coefficients(u, self.speed_coefficients())
    }

    fn length_from_coefficients(&self, u: T::Scalar, coefficients: (T::Scalar, T::Scalar, T::Scalar)) -> T::Scalar {
        let (c, b, a) = coefficients;
        let s0 = c.sqrt();
        if a.abs() == T::Scalar::from_i32(0) {
            u * s0
        } else {
            let z = a.sqrt();
            let n = T::Scalar::from_i32(1) / (T::Scalar::from_i32(2) * z);
            let p0 = b * n;
            let p1 = u * z + p0;
            let k = c - p0 * p0;
            let s1 = (p1 * p1 + k).sqrt();
            if k.abs() == T::Scalar::from_i32(0) {
                n * (p1 * s1 - p0 * s0)
            } else {
                n * (p1 * s1 - p0 * s0 + k * ((p1 + s1) / (p0 + s0)).ln())
            }
        }
    }

    pub fn length(&self, u: T::Scalar) -> T::Scalar {
        self.length_from_coefficients(u, self.speed_coefficients())
    }

    pub fn parameterize(&self, v: T::Scalar, tolerance: T::Scalar, max_iters: usize) -> Option<T::Scalar> {
        let coefficients = self.speed_coefficients();

        if coefficients.0 == T::Scalar::from_i32(0) && coefficients.1 == T::Scalar::from_i32(0) && coefficients.2 == T::Scalar::from_i32(0) {
            None
        } else {
            let mut l = T::Scalar::from_i32(0);
            let mut fl = -v;
            let mut r = T::Scalar::from_i32(1);
            let mut fr = self.length_from_coefficients(r, coefficients) - v;
            let mut x = l - (r - l) * fl / (fr - fl);

            if fl > T::Scalar::from_i32(0) {
                Some(T::Scalar::from_i32(0))
            } else if fr < T::Scalar::from_i32(0) {
                Some(T::Scalar::from_i32(1))
            } else {
                for _ in 0..max_iters {
                    let fx = self.length_from_coefficients(x, coefficients) - v;
                    if fx.abs() < tolerance {
                        break;
                    }

                    if fx > T::Scalar::from_i32(0) {
                        // root is to the left
                        r = x;
                        fr = fx
                    } else {
                        // root is to the right
                        l = x;
                        fl = fx;
                    }

                    // approximate next location of x using Newton's method
                    x -= fx / self.speed_from_coefficients(x, coefficients);
                    if x < l || x > r {
                        // if it falls out of range, use a linear approximation instead
                        x = l - (r - l) * fl / (fr - fl);
                    }
                }

                Some(x)
            }
        }
    }

    pub fn nearest_point(&self, q: T) -> T::Scalar {
        let a = self.x2.inner_product(self.x2);
        let b = self.x1.inner_product(self.x2) * T::Scalar::from_i32(2);
        let c = (self.x0 - q).inner_product(self.x2) * T::Scalar::from_i32(2) + self.x1.inner_product(self.x1);
        let d = (self.x0 - q).inner_product(self.x1) * T::Scalar::from_i32(2);
        let e = (self.x0 - q).inner_product(self.x0 - q);

        let distance = |t| (e + t * (d + t * (c + t * (b + a * t)))).sqrt();

        let left_value = T::Scalar::from_i32(0);
        let left_distance = distance(left_value);
        let right_value = T::Scalar::from_i32(1);
        let right_distance = distance(right_value);

        let (mut best_value, mut best_distance) = if left_distance <= right_distance {
            (left_value, left_distance)
        } else {
            (right_value, right_distance)
        };

        let roots = solve_cubic(T::Scalar::from_i32(4) * a, T::Scalar::from_i32(3) * b, T::Scalar::from_i32(2) * c, d);
        for &root in roots.as_slice().iter() {
            if root >= T::Scalar::from_i32(0) && root <= T::Scalar::from_i32(1) {
                let root_distance = distance(root);
                if root_distance < best_distance {
                    best_value = root;
                    best_distance = root_distance;
                }
            }
        }

        best_value
    }

    pub fn error_minimizing_linear_approximation(&self) -> LinearCurve<T> {
        LinearCurve::new(
            self.x0 - self.x2 / T::Scalar::from_i32(8),
            self.x1 + self.x2,
        )
    }

    pub fn continuity_preserving_linear_approximation(&self) -> LinearCurve<T> {
        LinearCurve::new(
            self.x0,
            self.x1 + self.x2,
        )
    }
}

#[cfg(test)]
mod tests {
    mod quadratic_curve {
        use crate::{
            vector2::Vector2,
            QuadraticCurve,
            InnerProductSpace,
            Scalar,
        };
        use approx::{
            assert_abs_diff_eq,
            assert_ulps_eq,
        };

        #[test]
        fn evaluate() {
            let curve = QuadraticCurve::new(1f32, 1f32, 1f32);

            assert_ulps_eq!(curve.evaluate_second_derivative(), 2f32);

            assert_ulps_eq!(curve.evaluate(0f32), 1f32);
            assert_ulps_eq!(curve.evaluate_first_derivative(0f32), 1f32);

            assert_ulps_eq!(curve.evaluate(0.5f32), 1.75f32);
            assert_ulps_eq!(curve.evaluate_first_derivative(0.5f32), 2f32);

            assert_ulps_eq!(curve.evaluate(1f32), 3f32);
            assert_ulps_eq!(curve.evaluate_first_derivative(1f32), 3f32);
        }

        #[test]
        fn bezier() {
            let test_cases = [
                (1f32, 1f32, 1f32),
                (2f32, 0f32, 1f32),
                (-1f32, 1f32, 0.5f32),
                (1f32, -1f32, -5f32),
                (6f32, -7f32, -9f32),
            ];
            for test_case in &test_cases {
                let curve = QuadraticCurve::from_bezier(test_case.0, test_case.1, test_case.2);
                let bezier = curve.to_bezier();
                assert_ulps_eq!(bezier.0, test_case.0);
                assert_ulps_eq!(bezier.1, test_case.1);
                assert_ulps_eq!(bezier.2, test_case.2);
            }
        }

        #[test]
        fn subdivide() {
            let curve = QuadraticCurve::new(1f32, 1f32, 1f32);
            let (left, right) = curve.subdivide(0.25f32);

            assert_ulps_eq!(left.x0(), 1f32);
            assert_ulps_eq!(left.x1(), 0.25f32);
            assert_ulps_eq!(left.x2(), 0.0625f32);

            assert_ulps_eq!(right.x0(), 1.3125f32);
            assert_ulps_eq!(right.x1(), 1.125f32);
            assert_ulps_eq!(right.x2(), 0.5625f32);
        }

        #[test]
        fn length() {
            let curve = QuadraticCurve::new(0f32, 1f32, 0f32);
            assert_ulps_eq!(curve.length(1f32), 1f32);

            let curve = QuadraticCurve::from_bezier(0f32, 2f32, 1f32);
            assert_ulps_eq!(curve.length(1f32), 5f32 / 3f32);

            let curve = QuadraticCurve::new(0f32, 0f32, 0f32);
            assert_ulps_eq!(curve.length(1f32), 0f32);

            let curve = QuadraticCurve::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 0f32), Vector2::new(0f32, 1f32));
            assert_ulps_eq!(curve.length(1f32), 1.4789429f32);
        }

        #[test]
        fn parameterize() {
            let curve = QuadraticCurve::new(0f32, 1f32, 0f32);
            check_curve_parameterization(curve);

            let curve = QuadraticCurve::from_bezier(0f32, 2f32, 1f32);
            check_curve_parameterization(curve);

            let curve = QuadraticCurve::new(Vector2::new(0f32, 0f32), Vector2::new(1f32, 0f32), Vector2::new(0f32, 1f32));
            check_curve_parameterization(curve);

            let curve = QuadraticCurve::new(0f32, 1f32, 0f32);
            assert_eq!(curve.parameterize(2f32, 0.001f32, 9), Some(1f32));
            assert_eq!(curve.parameterize(-1f32, 0.001f32, 9), Some(0f32));

            let test_cases = [
                (1f32, 1f32, 1f32),
                (2f32, 0f32, 1f32),
                (-1f32, 1f32, 0.5f32),
                (1f32, -1f32, -5f32),
                (6f32, -7f32, -9f32),
            ];
            for test_case in &test_cases {
                let curve = QuadraticCurve::from_bezier(test_case.0, test_case.1, test_case.2);
                check_curve_parameterization(curve);
            }
        }

        fn check_curve_parameterization<T: InnerProductSpace<Scalar = f32>>(curve: QuadraticCurve<T>) {
            const SAMPLES: i32 = 100;
            for i in 0..SAMPLES {
                let parameter = f32::from_i32(i) / f32::from_i32(SAMPLES);
                let length = curve.length(parameter);
                if curve.x2().inner_product(curve.x2()) == 0f32 && curve.x1().inner_product(curve.x1()) == 0f32 {
                    assert_eq!(curve.parameterize(length, 0.001f32, 9), None);
                } else {
                    assert_abs_diff_eq!(curve.length(curve.parameterize(length, 0.001f32, 9).unwrap()), length, epsilon = 0.001f32);
                }
            }
        }

        #[test]
        fn nearest_point() {
            let curve = QuadraticCurve::from_bezier(Vector2::new(0f32, 0f32), Vector2::new(0.5f32, 1f32), Vector2::new(1f32, 0f32));
            assert_ulps_eq!(curve.nearest_point(Vector2::new(0.5f32, 1f32)), 0.5f32);
            assert_ulps_eq!(curve.nearest_point(Vector2::new(-0.5f32, -0.5f32)), 0f32);
            assert_ulps_eq!(curve.nearest_point(Vector2::new(0.6f32, -0.5f32)), 1f32);
            assert_ulps_eq!(curve.nearest_point(Vector2::new(0.1f32, 0.3f32)), 0.15418649f32);

            let curve = QuadraticCurve::new(0f32, 0f32, 0f32);
            assert_ulps_eq!(curve.nearest_point(1f32), 0f32);
            assert_ulps_eq!(curve.nearest_point(0f32), 0f32);
            assert_ulps_eq!(curve.nearest_point(-1f32), 0f32);

            let curve = QuadraticCurve::new(Vector2::new(1f32, 1f32), Vector2::new(1f32, 1f32), Vector2::new(0f32, 0f32));
            assert_ulps_eq!(curve.nearest_point(Vector2::new(0f32, 0f32)), 0f32);
            assert_ulps_eq!(curve.nearest_point(Vector2::new(2.5f32, 2f32)), 1f32);
            assert_ulps_eq!(curve.nearest_point(Vector2::new(1f32, 2f32)), 0.5f32);
        }
    }
}