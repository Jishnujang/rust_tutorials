fn main() {
    // if / else: run code based on a condition.
    let number = 7;
    if number % 2 == 0 {
        println!("{number} is even");
    } else {
        println!("{number} is odd");
    }

    // else if: check multiple conditions.
    let score = 85;
    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else {
        println!("Grade: below B");
    }

    // if is an expression, so it can produce a value.
    let status = if number > 0 { "positive" } else { "zero or negative" };
    println!("The number is {status}");

    // loop repeats until break is reached.
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 3 {
            break counter;
        }
    };
    println!("The loop stopped at {loop_result}");

    // while repeats as long as its condition is true.
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}");
        countdown -= 1;
    }
    println!("Liftoff!");

    // for is useful for iterating over a range or collection.
    for number in 1..=5 {
        println!("Range value: {number}");
    }

    let colors = ["red", "green", "blue"];
    for color in colors {
        if color == "green" {
            println!("Color: {color}");
            break; // Stop after finding and printing green.
        }
    }
}
