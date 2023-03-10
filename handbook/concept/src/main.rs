fn main() {
    // mutable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // types
    let guess: u32 = "42".parse().expect("Not a Number");
}
