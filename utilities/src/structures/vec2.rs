use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, Mul, MulAssign, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn rotate_left(&mut self)
    where
        T: Neg<Output = T> + Clone,
    {
        let x = -self.y.clone();
        let y = self.x.clone();
        self.x = x;
        self.y = y;
    }
    pub fn rotate_right(&mut self)
    where
        T: Neg<Output = T> + Clone,
    {
        let x = self.y.clone();
        let y = -self.x.clone();
        self.x = x;
        self.y = y;
    }
    pub fn rotate_180(&mut self)
    where
        T: Neg<Output = T> + Clone,
    {
        let x = -self.x.clone();
        let y = -self.y.clone();
        self.x = x;
        self.y = y;
    }
}

// Add
impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
// BitAnd
impl<T: BitAnd<Output = T>> BitAnd for Vec2<T> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }
}
impl<T: BitAndAssign> BitAndAssign for Vec2<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.x &= rhs.x;
        self.y &= rhs.y;
    }
}
// Mul
impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl<T: MulAssign> MulAssign for Vec2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}
