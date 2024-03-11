fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This will throw an error, because x is immutable by default.
    println!("The value of x is: {}", x);
}
