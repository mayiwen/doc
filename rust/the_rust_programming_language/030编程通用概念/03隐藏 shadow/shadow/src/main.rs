fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The Value of x in the inner scope is: {}", x);
    }
    println!("The Value of x is: {}", x);
}

#[test]
fn test() {
    let space = "    ";
    let space = space.len();
    println!("len {}", space);
}

#[test]
fn test02() {
    let mut space = "    ";
    space = space.len(); // This length is in bytes, not [char]s or graphemes. In other words, it might not be what a human considers the length of the string.
    println!("len {}", space);
}
