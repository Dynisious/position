//! A 2D coordinate system.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/07/16

#![feature(const_fn)]

use std::{
    ops, borrow, hash, fmt,
    iter::{Sum, Iterator,},
};

#[cfg(feature = "pos-serde")]
extern crate serde;
#[cfg(feature = "pos-serde")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "pos-rand")]
extern crate rand as ext_rand;

mod traits;
pub mod rand;
mod tests;
mod pos_serde;

#[cfg(feature = "traits")]
pub use self::traits::*;

/// The origin of the coordinate system.
pub const ORIGIN: Pos = Pos::new(0, 0);
/// The unit vector of the X axis.
pub const X_UNIT: Pos = Pos::new(1, 0);
/// The unit vector of the Y axis.
pub const Y_UNIT: Pos = Pos::new(0, 1);

/// A 2D coordinate.
#[derive(PartialEq, Eq, Clone, Copy,)]
#[cfg_attr(feature = "pos-serde", derive(Serialize, Deserialize,))]
pub struct Pos<T = isize> {
    pub x: T,
    pub y: T,
}

impl<T> Pos<T> {
    pub const fn new(x: T, y: T) -> Self { Self { x, y, } }
}

impl<T> Pos<T>
    where T: ops::Mul<Output = T> + ops::Add<Output = T> {
    /// Calculates the dot product of the two [`Pos`] values.
    pub fn dot(a: Self, b: Self) -> T { (a.x * b.x) + (a.y * b.y) }
}

impl<T> Pos<T>
    where T: Clone + ops::Mul<Output = T> + ops::Add<Output = T> {
    /// Calculates the squared magniuted of this [`Pos`].
    pub fn mag2(self) -> T { Self::dot(self.clone(), self) }
}

impl<T> Pos<T>
    where T: Clone + ops::Mul<Output = T> + ops::Add<Output = T>
         + ops::Sub<Output = T> {
    /// Calculates the squared distance between this [`Pos`] and another.
    /// 
    /// # Params
    /// 
    /// pos --- The [`Pos`] value to get the distance too.
    pub fn dist2_from(self, pos: Self) -> T { (pos - self).mag2() }
}

impl<T> Pos<T>
    where T: Clone + ops::Mul<Output = T> + ops::Div<Output = T>
        + ops::Add<Output = T> {
    /// Calculates the part of `self` in the direction of `dir`.
    /// 
    /// # Params
    /// 
    /// dir --- The direction to get the part of `self` in.
    pub fn part(self, dir: Self) -> Self {
        dir.clone() * Self::dot(self, dir.clone()) / dir.mag2()
    }
}

impl<T> Pos<T>
    where T: Clone + ops::Mul<Output = T> + ops::Div<Output = T>
        + ops::Add<Output = T> + ops::Sub<Output = T> {
    /// Calculates the part of `self` perpendicular to `dir`.
    /// 
    /// # Params
    /// 
    /// dir --- The direction to get the part of `self` perpendicular too.
    pub fn complement(self, dir: Self) -> Self {
        self.clone() - self.part(dir)
    }
    /// Breaks `self` into its `(Part, Compliment)` relative to `dir`.
    /// 
    /// # Params
    /// 
    /// dir --- The direction to break `self` relative too.
    pub fn componenets(self, dir: Self) -> (Self, Self) {
        let part = self.clone().part(dir);

        (part.clone(), self - part)
    }
}

impl<T> Default for Pos<T>
    where T: Default {
    fn default() -> Self { Self::new(T::default(), T::default()) }
}

impl<T> From<(T, T)> for Pos<T> {
    #[inline]
    fn from((x, y): (T, T)) -> Self { Self::new(x, y) }
}

impl<T> Into<(T, T)> for Pos<T> {
    #[inline]
    fn into(self) -> (T, T) { (self.x, self.y) }
}

impl<T> ops::Neg for Pos<T>
    where T: ops::Neg {
    type Output = Pos<<T as ops::Neg>::Output>;

    fn neg(self) -> Self::Output { Pos::new(-self.x, -self.y) }
}

impl<T> ops::Add for Pos<T>
    where T: ops::Add {
    type Output = Pos<<T as ops::Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> ops::AddAssign for Pos<T>
    where T: ops::AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> ops::Sub for Pos<T>
    where T: ops::Sub {
    type Output = Pos<<T as ops::Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> ops::SubAssign for Pos<T>
    where T: ops::SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T, RHS> ops::Mul<RHS> for Pos<T>
    where T: ops::Mul<RHS>,
        RHS: Clone {
    type Output = Pos<<T as ops::Mul<RHS>>::Output>;

    fn mul(self, rhs: RHS) -> Self::Output { Pos::new(self.x * rhs.clone(), self.y * rhs) }
}

impl<T, RHS> ops::MulAssign<RHS> for Pos<T>
    where T: ops::MulAssign<RHS>,
        RHS: Clone {
    fn mul_assign(&mut self, rhs: RHS) {
        self.x *= rhs.clone();
        self.y *= rhs;
    }
}

impl<T, RHS> ops::Div<RHS> for Pos<T>
    where T: ops::Div<RHS>,
        RHS: Clone {
    type Output = Pos<<T as ops::Div<RHS>>::Output>;

    fn div(self, rhs: RHS) -> Self::Output { Pos::new(self.x / rhs.clone(), self.y / rhs) }
}

impl<T, RHS> ops::DivAssign<RHS> for Pos<T>
    where T: ops::DivAssign<RHS>,
        RHS: Clone {
    fn div_assign(&mut self, rhs: RHS) {
        self.x /= rhs.clone();
        self.y /= rhs;
    }
}

macro_rules! index_panic {
    ($index:expr) => (panic!("Index to [`Pos`] must be 0 (x) or 1 (y) gave {}.", $index))
}

impl<T> ops::Index<usize> for Pos<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => index_panic!(index),
        }
    }
}

impl<T> ops::IndexMut<usize> for Pos<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => index_panic!(index),
        }
    }
}

impl<T> hash::Hash for Pos<T>
    where T: hash::Hash {
    fn hash<H: hash::Hasher>(&self, h: &mut H) {
        for i in 0..2 {
            self[i].hash(h)
        }
    }
}

impl<T, A> Sum<A> for Pos<T>
    where T: Sum + ops::Add<Output = T> + Default + Clone,
        A: borrow::Borrow<Self> {
    fn sum<I>(iter: I) -> Self
        where I: Iterator<Item = A> {
        iter.fold(Pos::default(), |sum, pos| sum + pos.borrow().clone())
    }
}

impl<T> fmt::Display for Pos<T>
    where T: fmt::Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl<T> fmt::Debug for Pos<T>
    where T: fmt::Debug {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("({:?}, {:?})", self.x, self.y))
    }
}
