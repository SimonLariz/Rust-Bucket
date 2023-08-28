use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to temperature converter!");

    let mut user_input = String::new();

    while user_input.trim() != "quit"{
        user_input.clear();
        println!("Please enter a temperature in °F or 'quit' to quit.");
        print!(">");
        // flush() is needed to print! before read_line()
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let fahrenheit: f64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius = fahrenheit_to_celsius(fahrenheit);
        
        println!("{}°F is {:.2}°C", fahrenheit, celsius)
    }
    println!("Goodbye!");
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
