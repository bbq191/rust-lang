fn main() {
    let x = 5;
    println!("Unimmutable value of x is: {x}");
    let mut y = 5;
    println!("Before change mutable value of y is: {y}");
    y = 6;
    println!("After change mutable value of y is: {y}");
}
