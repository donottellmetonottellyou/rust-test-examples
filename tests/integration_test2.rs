use rust_test_examples::*;

#[test]
#[should_panic]
fn overflow() {
    add(usize::MAX, usize::MAX);
}
