    static  GLOBAL_COUNT: u32 = 670; //static variable, can be mutable, but must be declared in global scope

fn main() {

    let x = 5;//this variable immutable by default
    // x = 6; //this will give you the error because x is immutable
    let mut y = 10; //this variable is mutable
    y = 15; //this is allowed because y is mutable
    println!("Understanding variables in Rust!");

    /*RUST statically typed - compiler knows the type of each variable at compile time*/

    // we can also explicitly specify the type of a variable using a colon and the type name. For example, we can declare a variable of type i32 (32-bit signed integer) like this:
    let z: i32 = 20;
    println!("x:{}, y:{},z:{}",x,y,z);

    //const and static

    const MAX_VALUE: u32= 7000000; //constant variable, must be annotated with a type, and can be declared in any scope, including global scope
    //Use uppercase letters with underscores for constant names by convention.
    //Constants are immutable; you cannot change their value later.
    println!("MAX_VALUE:{}", MAX_VALUE);
    //static variables are similar to constants, but they have a fixed memory location and can be mutable. 
    // They are declared using the static keyword. For example:
    println!("GLOBAL_COUNT:{}", GLOBAL_COUNT);
}
