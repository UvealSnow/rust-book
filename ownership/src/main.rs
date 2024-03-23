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

    // This function takes ownership of the copy variable, once print_str is out of scope, so will copy.
    print_str(copy);

    // u32 implements the 'Copy' trait, meaning it can be trivially copied. It's still usable after print_int goes oos.
    print_int(x);

    let mine = gives_ownership();
    let mine = takes_and_gives(mine);

    println!("{}", mine);

    // The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. 
    // When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless
    // ownership of the data has been moved to another variable. If we don't want to grant ownership over a variable, we
    // can achieve this by passing a Reference of to the value we want to use.

    // References are a kind of non-owning pointer, they are created using the "&" operator, changing the variable type to &String (in this case),
    // meaning "a reference to a string", the arggument "s" does not own the variable "mine" or the heap data. Once "s" goes oos
    // no heap data is dealocated.
    let size = calculate_length(&mine);
    println!("The length of '{}' is {}", mine, size);

    // Box<T> is a pointer type that uniquely owns a heap allocation of type T.
    let mut x: Box<i32> = Box::new(1);              // 1 in Heap
    let a: i32 = *x;                                // a = 1 in Stack!
    *x += 1;

    let r1: &Box<i32> = &x;                         // r1 -> x -> 2 in Heap!
    let b = **r1;                              // 2 in Stack!

    let r2: &i32 = &*x;                             // Points directly to the Heap! (2)
    let c: i32 = *r2;                               // A single dereference is needed to get the value, 2 in the Stack!

    // The deference operator is not very often seen while reading Rust code, as the compiler automatically inserts deferences and
    // references, for example when calling a method using the dot notation. The following code is the same.
    let x_abs_1 = i32::abs(*x);             // Explicit deference
    let x_abs_2 = x.abs();                        // Implicit deference
    assert_eq!(x_abs_1, x_abs_2);

    let r = &x;
    let r_abs_1 = i32::abs(**r);          // Double explicit deference
    let r_abs_2 = r.abs();                      // Double implicit deference
    assert_eq!(r_abs_1, r_abs_2);

    let s = String::from("Hello");
    let s_len_1 = str::len(&s);         // Explicit reference
    let s_len_2 = s.len();                    // Implicit reference
    assert_eq!(s_len_1, s_len_2);

    // Data can be aliased and mutated, but never both at the same time.
    // This rule is enforced in the Box struc by default, when you try to asign a box to another variable it will move ownership and 
    // invalidate the previous variable. This ensures owned data can only be accessed through the owner - no aliases.
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num = &v[2];
    v.push(4);
    // println!("Third element is {}", *num);        // cannot borrow `v` as mutable because it is also borrowed as immutable

    // The borrow checker looks at 3 kinds of permissions on their data, Read, Write and Own (These don't exist in runtime)
    // By default a variable has RO permissions on its data, if the var is anotated with `let mut` then it'd have ROW permissions.
    // Permissions are defined on paths, not just variables. A path is anythingg you can put on a left hand assignment.
    // Rust uses these permissions in it's borrow checker to find potentially unsafe code. That's the reason why the code above 
    // will not compile, as `v.push(4);` requires v to have RW permissions, since borrowing v to num removes the W permission, 
    // the compiler will throw an error since the required permissions are not found.

    // Mutable references
    // The references we've seen so far are read-only immutable references (shared references), they permist aliasing but disalow
    // mutation. We can also provide mutable access to data w/o moving it.
    let mut v = vec![1, 2, 3];                  // v ^ RWO
    let num: &mut i32 = &mut v[2];                        // v -> , num ^ RO, *num ^ RW
    *num += 1;

    println!("The third element +1 is: {}", *num);        // v <- RWO, num - , *num -
    println!("Vector is now: {:?}", v);                   // v -

    // When num was an immutable reference, v still had the R permission. Now that num is a mutable reference, v has lost all permissions while num is in use.
    // When num was an immutable reference, the path *num only had the R permission. Now that num is a mutable reference, *num has also gained the W permission.

    // Data must outlive all of its references
    // To achieve this, Rust removes the O permission from the referenced data, if you try to drop a value while a reference still lives
    // an error will occurr.

    // Sumary:
    // 1. All variables can read, own, and (optionally) write their data.
    // 2. Creating a reference will transfer permissions from the borrowed path to the reference.
    // 3. Permissions are returned once the reference's lifetime has ended.
    // 4. Data must outlive all references that point to it.
}

fn print_str(str: String) {
    println!("{}", str);
} // Once we get here, str goes oos and 'drop' is called. The memory is freed.

fn print_int(int: u32) {
    println!("{}", int);
} // int goes oos here, it goes out with the stack, but since it was a copy it still exists in main.

// This function will move ownership of the returned String to the caller.
fn gives_ownership() -> String {
    String::from("your string")
}

// This function takes a String and retuns it to the caller.
fn takes_and_gives(str: String) -> String {
    str
}

// Borrows a refference to the String
fn calculate_length(s: &String) -> usize {
    s.len()
}
