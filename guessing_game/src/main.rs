// Program prelude, we can add more libraries as we go
use std::io;
// The Ordering type is an enum with variants Less, Greater, and Equal,
//  which are used to compare two values.
use std::cmp::Ordering;
// The Rng trait defines methods that random number generators implement, this trait comes from our dependencies.
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    // The thread_rng function returns a random number generator, which is local to the current 
    // thread of execution and seeded by the operating system.
    // I've decided to explicitly annotate the type of secret_number, but otherwise the compiler would correctly infer the type.
    let secret_number: u32 = rand::thread_rng()
        // The gen_range method of the random number generator, takes a range start..=end
        // and returns a random number within that range (inclusive of the lower and upper bounds).
        .gen_range(1..=100);

    // Only for debugging purposes!
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        // By default all variables in rust are immutable, unless the prefix `mut` is added.`
        // String is a type provided by the standard library that is a growable UTF-8 encoded bit of text.
        // String has an "associated function" ::new
        let mut guess = String::new();
    
        // Calling the stdin function from the io module, returns an instance of std::io::Stdin
        io::stdin()
            // The read_line method of the io::Stdin type, takes the input and appends it to the string, here passed by reference.
            // returns a Result type, which is an enum with variants Ok and Err.
            .read_line(&mut guess)
            // The expect method of the Result type, is used to handle the Result type, and will panic if the Result is an Err variant.
            .expect("Failed to read line");
    
        // Rust lets us shadow the previous value of guess with a new one, use this feature in situations in which 
        // you want to convert a value from one type to another type. In this case we're asserting the new type of guess
        // after the shadowing, which allows parse to infer the type it should parse to.
        // To handle the Result type returned by parse, we use a match expression.
        let guess: u32 = match guess.trim()
            // we could have also written .parse::<u32>(), similar to TS generics.
            .parse() {
                // The parse method returns OK, return the number and carry on.
                Ok(num) => num,
                // The parse method returns Err, "continue" means to go to the next iteration of the loop.
                Err(_) => continue,
            };
    
        // The {} is a placeholder for the value of the variable, and the value is passed as an argument to the println! macro.
        println!("You guessed: {guess}");
    
        // A match expression is made up of arms, each arm has two parts: a pattern and some code.
        // Similar to a switch statement in other languages, and anonymous functions in JS.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
