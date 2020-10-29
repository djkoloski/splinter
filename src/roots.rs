use crate::Scalar;

pub enum Roots<T: Scalar> {
    None([T; 0]),
    One([T; 1]),
    Two([T; 2]),
    Three([T; 3]),
}

impl<T: Scalar> Roots<T> {
    pub fn as_slice(&self) -> &[T] {
        match self {
            Roots::None(s) => &s[..],
            Roots::One(s) => &s[..],
            Roots::Two(s) => &s[..],
            Roots::Three(s) => &s[..],
        }
    }
}

pub fn solve_linear<T: Scalar>(a: T, b: T) -> Roots<T> {
    if a == T::from_i32(0) {
        Roots::None([])
    } else {
        Roots::One([-b / a])
    }
}

pub fn solve_quadratic<T: Scalar>(a: T, b: T, c: T) -> Roots<T> {
    if a == T::from_i32(0) {
        solve_linear(b, c)
    } else {
        let a2 = a * T::from_i32(2);
        let sqrtb2m4ac = (b * b - a * c * T::from_i32(4)).sqrt();
        let r0 = (-b + sqrtb2m4ac) / a2;
        let r1 = (-b - sqrtb2m4ac) / a2;
        Roots::Two([r0, r1])
    }
}

pub fn solve_cubic<T: Scalar>(a: T, b: T, c: T, d: T) -> Roots<T> {
    if a == T::from_i32(0) {
        solve_quadratic(b, c, d)
    } else {
        let a0 = d / a;
        let a1 = c / a;
        let a2 = b / a;

        let q = (T::from_i32(3) * a1 - a2 * a2) / T::from_i32(9);
        let r = (T::from_i32(9) * a2 * a1 - T::from_i32(27) * a0 - T::from_i32(2) * a2 * a2 * a2) / T::from_i32(54);

        let q3 = q * q * q;
        let d = q3 + r * r;
        let shift = -a2 / T::from_i32(3);

        if d >= T::from_i32(0) {
            let sqrtd = d.sqrt();
            let s = (r + sqrtd).cbrt();
            let t = (r - sqrtd).cbrt();

            let r1 = shift + s + t;

            if d == T::from_i32(0) {
                let r2 = shift - s;
                Roots::Two([r1, r2])
            } else {
                Roots::One([r1])
            }
        } else {
            let theta = (r / (-q3).sqrt()).acos();
            let k = T::from_i32(2) * (-q).sqrt();
            let r1 = k * (theta / T::from_i32(3)).cos() + shift;
            let r2 = k * ((theta + T::from_i32(2) * T::pi()) / T::from_i32(3)).cos() + shift;
            let r3 = k * ((theta - T::from_i32(2) * T::pi()) / T::from_i32(3)).cos() + shift;
            Roots::Three([r1, r2, r3])
        }
    }
}