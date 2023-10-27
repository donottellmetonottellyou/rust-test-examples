use super::*;

#[test]
#[should_panic]
fn dividing_by_zero_bad() {
    let _ = add2(3, 5) / add2(1, -1);
}
