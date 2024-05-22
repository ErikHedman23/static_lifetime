/*
Static Lifetimes - Indicates references available for entire duration of the program

ex: string literals

let s: &'static str = "Greetings!";

This can be coerced to more restrictive lifetime.


As a Trait Bound:

Ensures the data type will only contain 'static elements.

ex:
T: Display + 'static

*/
use std::fmt::Display;

fn print_display<T: Display + 'static>(item: T) {
    println!("{}", item);
}

fn main() {
    let s: &'static str = "Hello, world!";
    let num: &'static i32 = &42;

    print_display(s);
    print_display(num);
}
