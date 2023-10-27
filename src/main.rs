use rust_test_examples::add;

fn add2(left: isize, right: isize) -> isize {
    left + right
}

fn main() {
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("{a} + {b} = {c}");

    let a = 25;
    let b = 30;
    let c = add2(a, b);
    println!("{a} + {b} = {c}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add2(1, 5), 6);
    }
}

#[cfg(test)]
mod tests_dir_1;

#[cfg(test)]
#[path = "tests_dir_2/test1.rs"]
mod test1;

#[cfg(test)]
#[path = "../tests_dir_3/test2.rs"]
mod test2;
