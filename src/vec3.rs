use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::{random};

#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    #[inline]
    pub fn zeros() -> Vec3 {
        Vec3(0., 0., 0.)
    }

    #[inline]
    pub fn ones() -> Vec3 {
        Vec3(1., 1., 1.)
    }

    #[inline]
    pub fn random() -> Vec3 {
        (2. * Vec3(random(), random(), random()) - 1.).normalize()
    }

    #[inline]
    pub fn x(self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(self) -> f32 {
        self.1
    }

    #[inline]
    pub fn z(self) -> f32 {
        self.2
    }

    #[inline]
    pub fn r(self) -> f32 {
        self.0
    }

    #[inline]
    pub fn g(self) -> f32 {
        self.1
    }

    #[inline]
    pub fn b(self) -> f32 {
        self.2
    }

    #[inline]
    pub fn dot(self, other: Vec3) -> f32 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    #[inline]
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - other.1 * self.2,
            self.2 * other.0 - other.2 * self.0,
            self.0 * other.1 - other.0 * self.1,
        )
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn length_reciprocal(self) -> f32 {
        1.0 / self.length()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self * self.length_reciprocal()
    }
}

impl Default for Vec3 {
    #[inline]
    fn default() -> Vec3 {
        Vec3::zeros()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Vec3) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: f32) -> Vec3 {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Vec3) {
        self.0 = other.0;
        self.1 = other.1;
        self.2 = other.2;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: f32) -> Vec3 {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.0 = other;
        self.1 = other;
        self.2 = other;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: f32) -> Vec3 {
        Vec3(self.0 - other, self.1 - other, self.2 - other)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}
