// Functions 

// main function is the entry point of the program
// snake case is the convention for function names and variable names

fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameter(8);
    another_function_with_multiple_parameters(3, 'D');
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
