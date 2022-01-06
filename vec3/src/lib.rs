use std::ops;

#[derive(Debug, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub use Vec3 as Color;
pub use Vec3 as Point3;

#[macro_export]
macro_rules! v3 {
    ($x:expr, $y:expr, $z:expr) => {
        vec3::Vec3($x, $y, $z)
    };
}

use f64 as Ty;
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

    /// consume the vector and return a unit vector of the same direction
    pub fn unit_vector(self) -> Self {
        let l = self.length();
        self / l
    }
}

macro_rules! impl_binary_op {
    ($trait:ident, $method_name:ident, $op:tt) => {
        impl ops::$trait<Vec3> for Vec3 {
            type Output = Vec3;

            fn $method_name(self, rhs: Self) -> Self::Output {
                Vec3(self.0 $op rhs.0, self.1 $op rhs.1, self.2 $op rhs.2)
            }
        }
        impl ops::$trait<Ty> for Vec3 {
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
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl_binary_op!(Add, add, +);
impl_binary_op!(Sub, sub, -);
impl_binary_op!(Mul, mul, *);
impl_binary_op!(Div, div, /);
