// Functions 
// main function is the entry point of the program
// snake case is the convention for function names and variable names

fn main() {
    println!("Hello, world!");
    // function calls
    another_function();
    another_function_with_parameter(8);
    another_function_with_multiple_parameters(3, 'D');

    // statements and expressions
    // statements do not return values
    // let x = (let y = 6); // this will not compile
    let x = 5; // this is a statement that assigns the value 5 to x
    println!("The value of x is: {x}");

    // expressions evaluate to a resulting value
    let y = {
        let x = 3;
        x + 1 // this is an expression that evaluates to 4
        // expressions do not include ending semicolons 
    };
    println!("The value of y is: {y}");

    // functions with return values
    let x = six();
    println!("The value of x is: {x}");
}

// function definitions start with fn
fn another_function() {
    println!("Another function.");
}

// functions can have parameters but must declare the type
fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

// function with multiple parameters
fn another_function_with_multiple_parameters(x: i32, y: char) {
    println!("The value of x and y are: {x} and {y}");
}

// functions can return values
// the return type is declared after the arrow ->
fn six() -> i32 {
    6 // this is an expression that evaluates to 5
}