fn main() {
    /*Scalar types*/
    let whole_number: i32 = 42; // signed integer
    let count: u32 = 10; // unsigned integer
    let float_number: f64 = 3.14; // floating point number
    let is_active: bool = true; // boolean
    let character: char = 'J'; // character
    println!("whole_number: {}\ncount: {}\nfloat_number: {}\nis_active: {}\ncharacter: {}", whole_number, count, float_number, is_active, character);

    //Tuples : Groups of values of different types into a single compound type. 
    // Tuples have a fixed length and can contain elements of different types.
    // We can access the elements of a tuple using pattern matching or by using dot notation with the index of the element.
    // A tuple declared with `mut` can have its elements changed.
    let mut student: (&str, u8, f32) = ("John", 20, 3.5);
    student.1 = 21;
    println!("Student name: {}, age: {}, GPA: {}", student.0, student.1, student.2);

    //Arrays : Fixed-size collections of elements of the same type.
    // Arrays have a fixed length and can be accessed using an index.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 signed integers
    println!("last number: {}", numbers[4]);
    println!("Numbers: {:?}", numbers); // {:?} is used to print the array in debug format

    //Slices : Views into a contiguous sequence of elements in an array or a vector.
    // Slices are dynamically sized and can be used to borrow a portion of an array or vector without taking ownership of the data.
    let slice: &[i32] = &numbers[1..4]; // slice of the array from index 1 to 3 (exclusive)
    println!("Slice: {:?}", slice);

    //Vectors : Growable arrays that can change in size at runtime.
    // Vectors are implemented as a struct in the standard library and provide methods for adding, removing, and accessing elements.
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("Vector: {:?}", vector);

    // String owns growable UTF-8 text; &str is a borrowed string slice.
    let mut string: String = String::from("Hello");
    string.push_str(", world!");
    println!("String: {}", string);
    let greeting: &str = "Hello from a string slice";
    println!("&str: {}", greeting);

    //Enums : Custom data types that can have a fixed set of variants.
    // Enums are useful for representing a value that can be one of several possible types.
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    for direction in directions {
        match direction {
            Direction::Up => println!("Moving up"),
            Direction::Down => println!("Moving down"),
            Direction::Left => println!("Moving left"),
            Direction::Right => println!("Moving right"),
        }
    }

    // Structs: Custom data types that can have named fields of different types.
    // Structs are useful for representing a value that has multiple properties.
    struct Point {
        x: f32,
        y: f32,
    }
    let point = Point { x: 1.0, y: 2.0 };
    println!("Point: ({}, {})", point.x, point.y);

}
