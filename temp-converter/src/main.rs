use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to temperature converter!");

    let mut user_input = String::new();

    // Loop until user enters "quit"
    while user_input.trim() != "quit"{
        // Clear user_input
        user_input.clear();
        println!("Please enter a temperature in °F or 'quit' to quit.");
        print!(">");
        // flush() is needed to print! before read_line()
        io::stdout().flush().expect("Failed to flush stdout");

        // Read user input and remove trailing newline
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        // If user enters non-numeric input, continue loop
        let fahrenheit: f64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Convert fahrenheit to celsius
        let celsius = fahrenheit_to_celsius(fahrenheit);
        // Print result
        println!("{}°F is {:.2}°C", fahrenheit, celsius)
    }
    println!("Goodbye!");
}

// Convert fahrenheit to celsius and return result
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
