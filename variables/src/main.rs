use std::mem::size_of;
use rand::Rng;

// Constants are always immutable, they must be anotated and they can be declared in any scope, 
// including the global scope, they are valid for the entire time a program runs, within the scope they were declared.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Adding mut allows us to change the value of x.
    
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    let y = 6;
    let y = y + 6;
    {
        // Shadowing the value of x, allows us to change the type of x. In a sense the original x is "shadowed" by the new x
        // while this scope exists, y is redefined in the inner scope, and the original y is shadowed.
        // Shadowing allows us to perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
        // Compared to mut, shadowing has a few benefits. For example, because we’re effectively creating a new variable when we use the let keyword 
        // again, we can change the type of the value but reuse the same name.
        let y = x * 10;
        println!("The value of y is: {y}");
    }

    // The inner scope has ended, and the original y is now in scope.
    println!("The value of y is: {y}");

    // Rust needs to know the type of all variables at compile time, the type can be inferred from the value, or it can be explicitly annotated.
    // If we don't annotate the type, the compiler will try infer the type from the value. In this case, the compiler throws and error, because it can't infer the type.
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse()
        .expect("Not a number!");

    // Integer types in Rust:
    let unsigned_int_8: u8 = b'a'; // 0 to 255; = 97
    let unsigned_int_16: u16 = 65535; // 0 to 65535
    let unsigned_int_32: u32 = 4294967295; // 0 to 4294967295
    let unsigned_int_64: u64 = 18446744073709551615; // 0 to 18446744073709551615
    let unsigned_int_128: u128 = 340282366920938463463374607431768211455; // 0 to 340282366920938463463374607431768211455
    let unsigned_int_arch: usize = 18446744073709551615; // Depends on the architecture of the computer running the program.

    println!("The value of the unsigned integer is: {unsigned_int_8}");

    // Numbers can use a _ as a visual separator, for example 1_000 is the same as 1000.
    // We can also use any integer literals in Rust, such as 0x for hexadecimal, 0o for octal, and 0b for binary.
    let signed_int_8: i8 = -0x0f; // -128 to 127; = -15
    let signed_int_16: i16 = 32_767; // -32768 to 32767
    let signed_int_32: i32 = 2_147_483_647; // -2147483648 to 2147483647
    let signed_int_64: i64 = 9_223_372_036_854_775_807; // -9223372036854775808 to 9223372036854775807
    let signed_int_128: i128 = 0x7F_FFF_FFF_FFF_FFF_FFF_FFF_FFF_FFF_FFF_FFF; // -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
    let signed_int_arch: isize = 9223372036854775807; // Depends on the architecture of the computer running the program.

    println!("The value of the unsigned integer is: {signed_int_8}");

    let floating_point_32: f32 = 3.123456789; // 32-bit floating point number, max 7 digits, additional digits are rounded.
    let floating_point_64: f64 = 3.12345678901234567890; // 64-bit floating point number, max 16 digits, additional digits are rounded.

    println!("The value of the floating point number is: {floating_point_32}");
    println!("The value of the floating point number is: {floating_point_64}");

    // Boolean type in Rust:
    let t: bool = true; // this is trivially inferred by the compiler.
    // Boolean types are one byte in size.
    println!("bools: {}, u8s: {}", size_of::<[bool; 8]>(), size_of::<[u8; 8]>());

    // Character type in Rust:
    // 4 bytes in size, represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
    // they are specified with single quotes, as opposed to double quotes for strings.
    let c: char = 'z'; 
    println!("The value of the character is: {c}");
    let taco_emoji: char = '🌮';
    println!("The value of the taco emoji is: {taco_emoji}");
    let korean_char: char = '안';
    println!("The value of the Korean character is: {korean_char}");

    // Compound types in Rust:
    // Tuple type in Rust:
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // A tuple without any values is called a unit, and is a type and a value of its own, witten as (), repesents an empty return type.
    let rocket_emoji: char = '\u{1F680}';
    let tup: (i32, f64, u8, char) = (500, 6.4, 1, rocket_emoji); 
    let (x, y, z, r) = tup; // Destructuring a tuple.
    let rocket = tup.3; // Accessing a tuple element by index.
    println!("The value of y is: {y} {r}");

    // Array type in Rust:
    // An array is a collection of multiple values of the same type, and has a fixed length.
    // Arrays are useful when you want your data allocated on the stack rather than the heap.
    // To annotate the type of an array, we use square brackets, and the type of the elements, followed by a semicolon and the number of elements in the array.
    let a: [i8; 5] = [1, 2, 3, 4, 5];
    let mut rand_arr: [u32; 100] = [0; 100]; // An array of a hundred 0's.

    // We can do this because we're sending a slice, notice the [..] as the argument. 
    // This slice is a reference to the entire array.
    rand::thread_rng().fill(&mut rand_arr[..]);

    // Access an array element by index. If you try to access an index that is out of bounds, the program will panic.
    println!("The value of the first element in the random array is: {}, {}", rand_arr[0], rand_arr[1]);

    a_new_function();
    function_with_parameters(unsigned_int_32);
    let y = expression_example();
    println!("The value of y is: {y}");
    let p_one = plus_one(5);
    println!("The value of p_one is: {p_one}");
    let _test = plus_one;
    let text_p_one = _test(665);
    println!("The value of text_p_one is: {text_p_one}");

    // Control flow in Rust:
    // Conditionals MUST be boolean, if you try to use a non-boolean value, the compiler will throw an error.
    let number = _test(5);
    if number > 5 {
        println!("The value of p_one is greater than 5!");
    } else if number == 5 {
        println!("The value of p_one is equal to 5!");
    } else {
        println!("The value of p_one is less than or equal to 5!");
    }

    // Using if in a let statement:
    // The results of both arms of the if must be the same type.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Loops in Rust:
    // Rust has three kinds of loops: loop, while, and for.
    let mut i = 0;
    let res = loop {
        i += 1;
        if i == 2 {
            // Skip the rest of the iteration.
            continue;
        } else if i == 10 {
            // Exit the loop.
            break i * 2;
        } else {
            println!("The value of i is: {i}");
        }
    };

    println!("The value resulting of the loop is: {res}");

    let mut j = 0;
    // we can use labels to specify a particular loop that we want to break or continue.
    // Otherwise, the break or continue will apply to the innermost loop.
    'outer: loop {
        println!("The value of j is: {j}");
        j += 1;
        let mut k = 0;
        'inner: loop {
            println!("The value of k is: {k}");
            k += 1;
            if k == 5 {
                break 'inner;
            }
        }
        if j == 5 {
            break 'outer;
        }
    }

    let mut k = 5;
    while k != 0 {
        println!("The value of k in while loop is: {k}");
        k -= 1;
    }

    // We can use a for loop to iterate over a collection, such as an array.
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("The value of element in for loop is: {element}");
    }

    // We can use a for loop to iterate over a range of numbers.
    // Ranges are a type provided by the standard library, and they are notated with two dots.
    for number in (1..4).rev() {
        println!("The value of number in for loop is: {number}");
    }
}

// As a default, function names are snake case, and the parameters are annotated with their types.
// Rust does not care where we define the function, as long as it is defined somewhere.
fn a_new_function() {
    println!("This is a new function!");
}

fn function_with_parameters(x: u32) {
    println!("The value of x is: {x}");
}

fn expression_example() -> u8 {
    let x = 3;
    // The last line of a function is an expression, if you add the semicolon, it becomes a statement.
    // Expressions return a value, and statements do not.
    x + 1
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
