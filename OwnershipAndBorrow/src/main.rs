fn main() {
    // A String has one owner. Moving it transfers ownership to `second`.
    let first = String::from("hello");
    let second = first;
    println!("second owns the text: {second}");
    // `first` can no longer be used here because its ownership was moved.

    // Integers implement Copy, so assigning one copies its value.
    let original_number = 10;
    let copied_number = original_number;
    println!("Both numbers are usable: {original_number} and {copied_number}");

    // Borrowing lets a function read a value without taking ownership.
    let message = String::from("Rust");
    print_length(&message);
    println!("The owner can still use the text: {message}");

    // Multiple immutable borrows can be used at the same time.
    let first_read = &message;
    let second_read = &message;
    println!("Two reads: {first_read} and {second_read}");

    // Once those reads are no longer used, a mutable borrow can be made.
    let mut name = String::from("Address");
    {
        let name_ref = &mut name;
        add_last_name(name_ref);
    } // The mutable borrow ends here.
    println!("After the change: {name}");

    // A string slice borrows part of a String instead of copying it.
    let sentence = String::from("Rust makes memory safety practical");
    let first = first_word(&sentence);
    println!("First word: {first}");

    // Array slices borrow a range of elements.
    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4];
    println!("Middle numbers: {middle:?}");

    // A reference must not outlive the value it refers to.
    // This borrow is valid because `word` is used while `owned_text` is still in scope.
    let owned_text = String::from("scope");
    let word = first_word(&owned_text);
    println!("Borrowed word while its owner is alive: {word}");
}

fn print_length(text: &str) {
    println!("'{text}' has {} characters", text.len());
}

fn add_last_name(name: &mut String) {
    name.push_str(" Lovelace");
}

fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &text[..index];
        }
    }

    text
}
