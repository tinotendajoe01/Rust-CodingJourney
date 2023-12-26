fn main() {
    // Variables and Mutability
    let mut x = 5; // Declare a mutable variable 'x' and assign it the value 5
    println!("The value of x is: {}", x);
    x = 6; // Update the value of 'x' to 6
    println!("The value of x is now: {}", x);

    // Constants
    const MAX_VALUE: u32 = 100; // Declare a constant 'MAX_VALUE' with a value of 100
    println!("The maximum value is: {}", MAX_VALUE);

    // Shadowing
    let x = 5; // Declare a variable 'x' and assign it the value 5
    let x = x + 1; // Shadow the previous 'x' variable and assign it the value of 'x + 1'
    {
        let x = x * 2; // Shadow the previous 'x' variable within the inner scope and assign it the value of 'x * 2'
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x); // Access the outermost 'x' variable

    // Changing the type using Shadowing
    let spaces = "   "; // Declare a variable 'spaces' and assign it the value "   "
    let spaces = spaces.len(); // Shadow the previous 'spaces' variable and assign it the length of the string
    println!("Number of spaces: {}", spaces);
}