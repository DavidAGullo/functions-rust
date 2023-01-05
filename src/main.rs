//! Functions in Rust Example
//! Author: David Gullo
//! Date: 2022-12-21


#![allow(unused_variables)]
#![allow(unused_assignments)]

/// The Main Function is the entry point of the program
/// ```` fn main() {
///    println!("Functions Program Started");
/// }
/// ```` is the function signature
fn main() {
    let mut newstr = "David";
    say_hello_to(newstr);
    println!("Functions Program Started");
    for_loop();
    say_hello_to_by_ref(&mut newstr);
    println!("{}", newstr);
    let greeting = say_hello2(&mut newstr);
    println!("{}", greeting);
}

/// This Function prints Hello to the console
/// ```` fn say_hello() {
///   println!("Hello");
/// }
/// ```` is the function signature
fn say_hello() {
    println!("Hello");
}

/// This Function loops through a range of numbers and calls the say_hello() function
/// ```` fn for_loop() {
///   for i in 1..10 {
///      say_hello();
///  }
/// }
/// ```` is the function signature
fn for_loop() {
    for i in 1..3 {
        say_hello();
    }
}

/// This Function takes a string as a parameter and prints it to the console
/// the &str is a reference to a string which is found in the main() function
/// ```` fn say_hello_to(name: &str) {
///  println!("Hello {}", name);
/// }
/// ```` is the function signature
fn say_hello_to(name: &str) {
    println!("Hello {}", name);
}

/// This function passes paremeters by reference
/// 
/// ```` fn say_hello_to_by_ref(name: &mut &str) {
/// println!("Hello {}", name);
/// }
/// ```` is the function signature

fn say_hello_to_by_ref(name: &mut &str) {
    *name = "Nick";
    println!("Hello {}", name);
}

fn say_hello2(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    return greeting;
}