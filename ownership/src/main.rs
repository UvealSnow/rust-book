use std::rc::Rc;

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

// Fixing ownership errors

// The following code has a simple ownership issue, the data it borrows from goes oos before the reference.
// fn buggy_return_a_string() -> &String {
//     let s = String::from("Hello");
//     &s
// }

// One way to fix it is by returning the value itself
fn return_a_string_value() -> String {
    String::from("Hello")
}

// We could also return a string literal stored in the stack instead of the heap
fn return_a_string_ref() -> &'static str {
    "Hello"
}

// We can also use a Reference Counted (Rc) pointer, Rc::clone only copies the reference, not the data.
// At runtime Rc checks for the last reference to the data and then dealocates it.
fn return_a_rc_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello"));
    Rc::clone(&s)
}

// The consumer could also just send us a place to store our String. Making the caller responsible of
// memora allocation.
fn return_string_to_slot(out: &mut String) {
    out.replace_range(.., "Hello")
}

// The following code is missing runtime permissions
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

// We could just pass a mutable reference, but this is not a good solution as functions shoulf NOT modify
// their inputs when the caller is not expecting it.
fn stringify_mutable_name_with_title(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// We could take ownership of the name Vector, but this is also not great as functions should try not to take
// ownership of heap-owning data structures, also name will be unusable after the call.
fn stringify_my_name_with_title(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// Using a mutable reference is the best way then, we should copy it to avoid changing the original
// In this case, we copy the name by using slice.join()
fn stringify_copied_name_with_title(name: &mut Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

// The following code fails because dst does not have W permissions due to it being referenced by largest.
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

// We can remove the dst reference altogether by using the length, since we don't need the contents of the String
// This is a performant and idiomatic solution, as we're not copying values from the stack and we shorten the lifetime
// of borrows so it can be modified.
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len = dst.iter().max_by_key(|s| s.len())
        .unwrap()
        .len();

    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn confusing_stack_and_heap() {
    // This code is fine because we're using primitive values that don't own data in the heap
    let v = vec![1, 2, 3];
    let v_ref: &i32 = &v[0];
    let n = *v_ref;

    // This code is not safe bc String owns data in the heap that cannot be copied without moving.
    // When s goes oos, the String it's pointing to is dealocated, v_ref and v think they own the non-existent
    // String at this point, then v_ref goes oos and tries to dealocate the String.
    let v = vec![String::from("Hello world")];
    let v_ref: &String = &v[0];
    // let s: String = *v_ref;                         // cannot move out of `*v_ref` which is behind a shared reference

    // As an exception, to follow anti-aliasingg policy, an immutable reference cannot be used after it's reasigned 
    // let mut n = 0;
    // let a = &mut n;
    // let b = a;                                      // a is no longer valid from this point

    // There are many ways to make that code safe:
    // 1. Avoid taking ownership of the vector
    let v = vec![String::from("Hello world")];
    let s_ref = &v[0];
    println!("{s_ref}");

    // 2. Clone the data
    let v = vec![String::from("Hello world")];
    let mut s = v[0].clone();
    s.push('!');
    println!("{s}");
    
    // 3. Take the data out of the vectror
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s = v.remove(0);
    s.push('!');
    println!("{s}");
}

fn fixing_safe_code_1() {
    // The borrow checker has limitations, this code is completely safe as we're just using a reference to 
    // a single path of the tuple, in fact if we do this inline it works fine. But when we use closures or functions
    // it looses track of the fine grained permissions as it will not check the output of the function, it'll just assume
    // the whole tuple is referenced as there's no way for it to know which of the two Strings is being referenced.
    fn get_first(tuple: &(String, String)) -> &String {
        &tuple.0
    }

    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean"),
    );

    let first = get_first(&name);

    // name.1.push_str(" Esq.");               // cannot borrow `name.1` as mutable because it is also borrowed as immutable
    println!("{first} {}", name.1);
}

fn fixing_safe_code_2() {
    // This code fails because Rust will not check every path for the array a, assumes that if we allowed x to make a single-ref
    // on a, then all paths of a are mutable references, so the 
    let mut a = [0, 1, 2, 3];
    let x = &mut a[2];
    // let y = &a[2];                      // cannot borrow `a[_]` as immutable because it is also borrowed as mutable
    // *x += *y;

    // We could use slice::split_at_mut that implements unsafe blocks to do their work, we can do the same.
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[0];
    let y = &a_r[0];
    *x += *y;

    // We must be careful using unsafe blocks
    let x = &mut a[0] as *mut i32;
    let y = &a[2] as *const i32;
    unsafe { // Defeats the purpose of using Rust unless you KNOW your code is safe.
        *x += *y;
    } 
    // Unsafe code is sometimes necessary to work around the limitations of the borrow checker.
    // unsafe code is heavily used in the std library, it's how Rust implements certain otherwise-impossible patterns.
}

fn point() {
    let mut point = [0, 1];
    let mut x = point[0];
    let y = &mut point[1];
    x += 1;
    *y += 1;
    println!("{} {}", point[0], point[1]);
}