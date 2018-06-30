
#![cfg(test)]

use super::*;

#[test]
fn test_pos() {
    let one = Pos::new(1, 1);
    let two = Pos::new(2, 2);
    let pos = Pos::new(1, 2);
    
    assert_eq!(Pos::dot(one, two), 4, "`Pos::dot` failed.");
    assert_eq!(pos.mag2(), 5, "`Pos::mag2` failed.");
    assert_eq!(one.dist2_from(two), 2, "`Pos::dist2_from` failed.");
    assert_eq!(pos.part(x_unit()), x_unit(), "`Pos::part 1` failed.");
    assert_eq!(pos.part(y_unit()), y_unit::<usize>() * 2, "`Pos::part 2` failed.");
    assert_eq!(pos.complement(y_unit()), x_unit(), "`Pos::complement 1` failed.");
    assert_eq!(pos.complement(x_unit()), y_unit::<usize>() * 2, "`Pos::complement 2` failed.");
    assert_eq!(pos.componenets(x_unit()), (x_unit(), y_unit::<usize>() * 2), "`Pos::components 1` failed.");
    assert_eq!(pos.componenets(y_unit()), (y_unit::<usize>() * 2, x_unit()), "`Pos::components 2` failed.");
}

#[test]
fn test_pos_arithmetic() {
    let one = Pos::new(1, 1);
    let two = Pos::new(2, 2);

    assert_eq!(Pos::default(), origin::<usize>(), "`Pos::default` unexpected value.");
    assert_eq!(one + one, two, "`Pos::add` failed.");
    assert_eq!(two - one, one, "`Pos::sub` failed.");
    assert_eq!(one * 2, two, "`Pos::mul` failed.");
    assert_eq!(two / 2, one, "`Pos::div` failed.");

    let mut temp = one;

    temp += one;
    assert_eq!(temp, two, "`Pos::add_assign` failed.");
    temp -= one;
    assert_eq!(temp, one, "`Pos::sub_assign` failed.");
    temp *= 2;
    assert_eq!(temp, two, "`Pos::mul_assign` failed.");
    temp /= 2;
    assert_eq!(temp, one, "`Pos::div_assign` failed.");
}

#[cfg(feature = "pos-rand")]
#[test]
fn test_pos_rand() {
    Pos::<isize>::random();
}
