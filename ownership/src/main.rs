fn main() {
    // Ownership rules in Rust:
    // - Each value in Rust has an owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it's not yet declared.
        let s = "hello"; // s is valid from this point on.
        // Do stuff with s ...
    } // scope is now over, s is no longer valid.

    // this will not work, string literals are immutable. We know their size at compile time, so it's directly
    // hardcoded into the executable. This makes string literals fast and efficient, but we can't mutate them.
    // let mut a = "Hello";
    // a.push_str(", world!");
    // println!("{}", a);

    // The type String allocates to the heap, so we can store an amount of text that is unknown at compile time.
    let mut s = String::from("Hello");
    s.push_str(", world!");

    println!("{}", s);

    // As we know the size of these integers at compile time, the value is copied to the stack, 
    // so we have the number "5" in the stack twice.
    let x = 5;
    let y = x;

    // In this case, String is a complex type that is conformed of three parts, a pointer to the heap, lenght and capacity.
    // When we copy s to str it copies these 3 values, the pointer points to the same  but it will not duplicate the data in the heap.
    let mut str = s;
    str.push_str("\nI'm Kevin");

    // This fails, as Rust considers s as no longer valid. The process of copying this variable is called a "move" as opposed to 
    // shallow copy because the first variable s is also invalidated. When str goes out of scope it alone will free the memory.
    // println!("{}", s);

    // Rust will never automatically perform deep copies of data. 
    // A way to get this behaviour is with the .clone() method, effectively it duplicates the data in the heap.
    // It's a way to explicitly show we're executing some arbitrary, possibly expensive code.
    // Rust has a special annotation called the 'Copy' trait. f a type implements the Copy trait, variables that use 
    // it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
    // We can't use Copy if we have used the 'Drop' trait - If the type needs something special to happen when the 
    // value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.
    let copy = str.clone();
    println!("{}\n{}", str, copy);
}
