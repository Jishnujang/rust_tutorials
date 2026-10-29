fn main() {
    let mut student = Student::new("Maya", 20);

    // Methods using &self can read the struct without taking ownership.
    student.introduce();
    println!("Is this Maya? {}", student.has_name("Maya"));

    // Methods using &mut self can change the struct.
    student.have_birthday();
    student.introduce();
}

struct Student {
    name: String,
    age: u8,
}

impl Student {
    // An associated function has no self parameter; use it to create a Student.
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: String::from(name),
            age,
        }
    }

    fn introduce(&self) {
        println!("Hi, I'm {} and I'm {} years old.", self.name, self.age);
    }

    fn has_name(&self, name: &str) -> bool {
        self.name == name
    }

    fn have_birthday(&mut self) {
        self.age += 1;
    }
}
