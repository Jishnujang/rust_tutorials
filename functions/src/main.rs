fn main() {
    // Call a function with no parameters and no return value.
    greet();

    // Pass arguments to a function with parameters.
    let name1 = "Alex";
    greet_person(name1);

    // Capture the value returned by a function.
    let result = add(5, 3);
    println!("5 + 3 = {result}");

    // The final expression is returned without a semicolon.
    let doubled = double(4);
    println!("Double 4 = {doubled}");
}

fn greet() {
    println!("Hello!");
}

fn greet_person(name: &str) {
    println!("Hello, {name}!");
}

fn add(left: i32, right: i32) -> i32 {
    let sum = left + right;
    println!("Adding {left} and {right}");
    sum
}

fn double(number: i32) -> i32 {
    number * 2
}
