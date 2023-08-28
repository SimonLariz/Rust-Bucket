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

    // while loops are similar to other languages
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // looping through a collection with while
    println!("Looping through a collection with while");
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // for loops are used to loop through a collection
    // for loops are cleaner than while loops
    println!("Looping through a collection with for");
    let a = [10,20,30,40,50];

    for element in a {
        println!("the value is: {}", element);
    }
    
    // for loops can also be used to loop through a range
    println!("Looping through a range with for");
    // .rev reverses the range
    // range is inclusive on the start and exclusive on the end
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
