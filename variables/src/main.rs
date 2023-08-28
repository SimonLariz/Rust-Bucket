fn main() {
    // Variables and Mutability
    // Variables are immutable by default, but can be made mutable with the mut keyword
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    // Constants are always immutable, and must be annotated with a type
    // Naming convention is to use all uppercase with underscores between words
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // Shadowing allows us to redeclare a variable with the same name
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Shadowing allows us to change the type of a variable
    // mut does not allow this, since we can not mutate the type of a variable
    let spaces = "   ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // Data Types
    // Rust is a statically typed language, meaning it must know the types of all variables at compile time

    // Scalar Types
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
    let integer: u32 = 5;
    let float: f32 = 5.8;
    let boolean: bool = true;
    let character: char = 'a';
    println!("The values of integer, float, boolean, and character are: {integer}, {float}, {boolean}, {character}");

    // Compound Types
    // Compound types can group multiple values into one type
    // Rust has two primitive compound types: tuples and arrays
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values of x, y, and z are: {x}, {y}, {z}");

}
