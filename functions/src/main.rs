// Functions 

// main function is the entry point of the program
// snake case is the convention for function names and variable names

fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameter(8);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}
