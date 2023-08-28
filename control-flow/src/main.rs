// Control Flow
fn main() {
    let number = 3;

    // if expressions are similar to other languages
    // but they are expressions, not statements
    // condition must be a bool
    if number < 5 {
        println!("Condition was true");
    }
    else {
        println!("Condition was false");
    }

    // handle multiple conditions with else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expression, so it can be used on the right side of a let statement
    let condition = true;
    let number = if condition {
        5
    }
    else {
        6
    };
    println!("The value of number is: {}", number);

    // Rust has three kinds of loops: loop, while, and for
    // loop executes code until you tell it to stop
    // break and continue work as expected
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    // return value of loop can be used to return a value from the loop
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 3;
        }
    };
    println!("The result is {result}");

    // loop labels can be used to break or continue outer loops
    let mut count = 0;
    // 'counting_up is the label for the outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // break the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
