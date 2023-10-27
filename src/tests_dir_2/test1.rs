use super::*;

#[test]
#[should_panic]
fn underflow() {
    let _ = add2(isize::MIN, isize::MIN);
}
