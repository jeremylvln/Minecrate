use std::ops::{Add, Sub, Mul, Div};
use num::Integer;

#[allow(dead_code)]
pub struct Vector2<T>(pub T, pub T);
#[allow(dead_code)]
pub type Vector2i = Vector2<i32>;
#[allow(dead_code)]
pub type Vector2u = Vector2<u32>;

#[allow(dead_code)]
pub struct Vector3<T>(pub T, pub T, pub T);
#[allow(dead_code)]
pub type Vector3i = Vector3<i32>;
#[allow(dead_code)]
pub type Vector3u = Vector3<u32>;

impl<T> Vector2<T> {
    #[allow(dead_code)]
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Integer + Copy> Add<T> for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs)
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Integer + Copy> Sub<T> for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs)
    }
}

impl<T: Integer + Copy> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs)
    }
}

impl<T: Integer + Copy> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs)
    }
}

impl<T> Vector3<T> {
    #[allow(dead_code)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }
}

impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Integer + Copy> Add<T> for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Integer + Copy> Sub<T> for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl<T: Integer + Copy> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<T: Integer + Copy> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
