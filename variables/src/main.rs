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
}
