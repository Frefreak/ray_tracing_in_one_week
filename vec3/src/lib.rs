use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Color = Vec3;
pub type Point3 = Vec3;

#[macro_export]
macro_rules! v3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::Vec3($x, $y, $z)
    };
}

use f64 as Ty;
use utils::{random_double, random_double_range};
impl Vec3 {
    pub fn x(&self) -> Ty {
        self.0
    }
    pub fn y(&self) -> Ty {
        self.1
    }
    pub fn z(&self) -> Ty {
        self.2
    }

    pub fn length(&self) -> Ty {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Ty {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    /// dot product
    pub fn dot(&self, rhs: &Self) -> Ty {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    /// cross product
    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn sqrt(&self) -> Self {
        Vec3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt())
    }

    /// consume the vector and return a unit vector of the same direction
    pub fn unit_vector(self) -> Self {
        let l = self.length();
        self / l
    }

    pub fn random() -> Self {
        Vec3(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: Ty, max: Ty) -> Self {
        Vec3(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.0.abs() < S && self.1.abs() < S && self.2.abs() < S
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1., 1.);
        if p.length_squared() >= 1. {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1., 1.);
        if p.length_squared() >= 1. {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = v3!(random_double_range(-1., 1.), random_double_range(-1., 1.), 0.);
        if p.length_squared() >= 1. {
            continue;
        }
        return p;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

/// From: <https://raytracing.github.io/books/RayTracingInOneWeekend.html>
///
/// In order to determine the direction of the refracted ray, we have to solve for
/// $\sin\theta^\prime$:
/// $$\sin \theta^\prime = \frac{\eta}{\eta^\prime}\cdot \sin \theta $$
/// On the refracted side of the surface there is a refracted ray
/// $\mathbf{R}^\prime$ and a normal $\mathbf{n}^\prime$, and there
/// exists an angle, $\theta^\prime$, between them. We can split
/// $\mathbf{R}^\prime$ into the parts of the ray that are perpendicular
/// to $\mathbf{n}^\prime$ and parallel to $\mathbf{n}^\prime$:
/// $$ \mathbf{R}^\prime = \mathbf{R}^\prime_\perp +
/// \mathbf{R}^\prime_\parallel$$
/// If we solve for $\mathbf{R}^\prime_\perp$ and $\mathbf{R}^\prime_\parallel$
/// we get:
/// $$ \mathbf{R}^\prime_\perp = \frac{\eta}{\eta^\prime}
/// (\mathbf{R} + \cos \theta\mathbf{n}) $$
/// $$ \mathbf{R}^\prime_\parallel = -\sqrt{1 - |\mathbf{R}^\prime_\perp|^2
/// }\mathbf{n}$$
/// We still need to solve for $\cos \theta$. It is well known that the
/// dot product of two vectors can be explained in terms of the cosine of
/// the angle between them:
/// $$ \mathbf{a} \cdot \mathbf{b} = |\mathbf{a}||\mathbf{b}|\cos \theta $$
/// If we restrict $\mathbf{a}$ and $\mathbf{b}$ to be unit vectors:
/// $$ \mathbf{a}\cdot\mathbf{b} = \cos \theta$$
/// We can now write $\mathbf{R}^\prime_\perp$ in terms of known quantities:
/// $$ \mathbf{R}^\prime_\perp = \frac{\eta}{\eta^\prime}
/// (\mathbf{R} + (-\mathbf{R}\cdot\mathbf{n})\mathbf{n}) $$
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
    return r_out_parallel + r_out_perp;
}

macro_rules! impl_binary_op {
    ($trait:ident, $method_name:ident, $op:tt) => {
        impl ops::$trait<Vec3> for Vec3 {
            type Output = Vec3;

            fn $method_name(self, rhs: Self) -> Self::Output {
                Vec3(self.0 $op rhs.0, self.1 $op rhs.1, self.2 $op rhs.2)
            }
        }
        impl ops::$trait<&Vec3> for Vec3 {
            type Output = Vec3;

            fn $method_name(self, rhs: &Self) -> Self::Output {
                Vec3(self.0 $op rhs.0, self.1 $op rhs.1, self.2 $op rhs.2)
            }
        }
        impl ops::$trait<Vec3> for &Vec3 {
            type Output = Vec3;

            fn $method_name(self, rhs: Vec3) -> Self::Output {
                Vec3(self.0 $op rhs.0, self.1 $op rhs.1, self.2 $op rhs.2)
            }
        }
        impl<'a, 'b> ops::$trait<&'b Vec3> for &'a Vec3 {
            type Output = Vec3;

            fn $method_name(self, rhs: &'b Vec3) -> Self::Output {
                Vec3(self.0 $op rhs.0, self.1 $op rhs.1, self.2 $op rhs.2)
            }
        }
        impl ops::$trait<Ty> for Vec3 {
            type Output = Vec3;

            fn $method_name(self, rhs: Ty) -> Vec3 {
                Vec3(self.0 $op rhs, self.1 $op rhs, self.2 $op rhs)
            }
        }
        impl ops::$trait<Ty> for &Vec3 {
            type Output = Vec3;

            fn $method_name(self, rhs: Ty) -> Vec3 {
                Vec3(self.0 $op rhs, self.1 $op rhs, self.2 $op rhs)
            }
        }
        impl ops::$trait<Vec3> for Ty {
            type Output = Vec3;

            fn $method_name(self, rhs: Vec3) -> Vec3 {
                Vec3(self $op rhs.0, self $op rhs.1, self $op rhs.2)
            }
        }
        impl ops::$trait<&Vec3> for Ty {
            type Output = Vec3;

            fn $method_name(self, rhs: &Vec3) -> Vec3 {
                Vec3(self $op rhs.0, self $op rhs.1, self $op rhs.2)
            }
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl_binary_op!(Add, add, +);
impl_binary_op!(Sub, sub, -);
impl_binary_op!(Mul, mul, *);
impl_binary_op!(Div, div, /);

#[cfg(test)]
mod test {
    #[test]
    fn test_op() {
        let v1 = v3!(1., 2., 3.);
        let v2 = v3!(4., 5., 6.);
        assert_eq!(v1 + v2, v3!(5., 7., 9.));
        assert_eq!(v1 - v2, v3!(-3., -3., -3.));
        assert_eq!(v1 * v2, v3!(4., 10., 18.));
        assert_eq!(v1 / v2, v3!(0.25, 0.4, 0.5));
        assert_eq!(v1 * 3., v3!(3., 6., 9.));
    }
}

