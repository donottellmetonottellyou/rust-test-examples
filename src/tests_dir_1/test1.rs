use super::*;

#[test]
fn negative_is_one_bigger_than_positive() {
    assert_eq!(add2(isize::MIN, isize::MAX), -1)
}
