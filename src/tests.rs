
use super::{ORIGIN, X_UNIT, Y_UNIT, Pos,};

#[test]
fn test_pos() {
    let one = Pos::new(1, 1);
    let two = Pos::new(2, 2);
    let pos = Pos::new(1, 2);
    
    assert_eq!(Pos::dot(one, two), 4, "`Pos::dot` failed.");
    assert_eq!(pos.mag2(), 5, "`Pos::mag2` failed.");
    assert_eq!(one.dist2_from(two), 2, "`Pos::dist2_from` failed.");
    assert_eq!(pos.part(X_UNIT), X_UNIT, "`Pos::part 1` failed.");
    assert_eq!(pos.part(Y_UNIT), Y_UNIT * 2, "`Pos::part 2` failed.");
    assert_eq!(pos.complement(Y_UNIT), X_UNIT, "`Pos::complement 1` failed.");
    assert_eq!(pos.complement(X_UNIT), Y_UNIT * 2, "`Pos::complement 2` failed.");
    assert_eq!(pos.componenets(X_UNIT), (X_UNIT, Y_UNIT * 2), "`Pos::components 1` failed.");
    assert_eq!(pos.componenets(Y_UNIT), (Y_UNIT * 2, X_UNIT), "`Pos::components 2` failed.");
}

#[test]
fn test_pos_arithmetic() {
    let one = Pos::new(1, 1);
    let two = Pos::new(2, 2);

    assert_eq!(Pos::default(), ORIGIN, "`Pos::default` unexpected value.");
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
