use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct GenPoint<T: Copy + Clone>(pub T, pub T);

impl<T: Add + Copy + Clone> GenPoint<T> {
    pub fn sum(&self) -> <T as Add>::Output {
        self.0 + self.1
    }
}

impl<T> GenPoint<T> where
    T: Sub + Copy + Clone
{
    pub fn sub(&self) -> <T as Sub>::Output {
        self.0 - self.1
    }
}

// impl Add<Output=u32> for GenPoint<u32> {
//     fn add(self, rhs: Self) -> u32 {
//         return 4
//     }

// }