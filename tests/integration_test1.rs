use rust_test_examples::*;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
#[should_panic]
fn it_does_not_work() {
    panicky_function();
}
