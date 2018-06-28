
#![cfg(feature = "traits")]

use super::Pos;
use std::ops::AddAssign;
use std::borrow::{Borrow, BorrowMut,};

/// A `GetPos` type is something which can be considered to have a position in a 2D
/// coordinate space.
pub trait GetPos<T = i32>: Sized {
    /// Returns the position of `self`.
    fn get_position(&self) -> Pos<T>;
}

impl<Type, T> GetPos<T> for Type
    where Type: Borrow<Pos<T>>,
        T: Clone {
    fn get_position(&self) -> Pos<T> { self.borrow().clone() }
}

/// A `Position` type is something which can be considered to have a dynamic position in
/// a 2D coordinate space.
pub trait Position<T = i32>: GetPos<T>
    where T: AddAssign {
    /// Sets the position of `self`.
    fn set_position(self, pos: Pos<T>) -> Self;
    /// Translates `self` by the given translation.
    fn translate(self, mut trans: Pos<T>) -> Self {
        trans += self.get_position();

        self.set_position(trans)
    }
}

impl<Type, T> Position<T> for Type
    where Type: BorrowMut<Pos<T>>,
        T: Clone + AddAssign {
    fn set_position(mut self, pos: Pos<T>) -> Self { *self.borrow_mut() = pos; self }
}

#[cfg(test)]
mod tests {
    use super::{super::Pos, GetPos, Position};

    #[test]
    fn test_traits() {
        let pos = Pos::<i32>::default();
        let other = Pos::new(1, 2);

        assert_eq!(pos.get_position(), Pos::default(), "`Position::get_position` failed.");
        assert_eq!(pos.set_position(other), other, "`Position::set_position` failed.");
        assert_eq!(other.translate(other), other * 2, "`Position::translate` failed.");
    }
}
