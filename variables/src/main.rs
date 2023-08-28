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

    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values of x, y, and z are: {x}, {y}, {z}");
    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");
    // Empty tuples are also a type, and are written as () or Unit

    // Arrays
    // Arrays in Rust are FIXED in length, and can only contain elements of the same type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // We can access an array element by using square brackets and the index of the value we want to access
    let third = arr[3];
    println!("The value of third is: {third}");
    // We can also initialize an array with the same value for each element
    let arr = [3; 5];
    // This creates an array with 5 elements, each with the value 3
    // Note: Rust will panic if we try to access an array element that does not exist

}
