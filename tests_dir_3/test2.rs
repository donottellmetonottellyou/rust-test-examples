// This tests file is outside of src/! Check out main.rs to see how.
use super::*;

#[test]
fn not_the_same() {
    let a = add(3, 7);
    assert_ne!("Rust", "C++");

    // Will not show by default
    println!("{a}");
}
