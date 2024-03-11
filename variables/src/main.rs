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
}
