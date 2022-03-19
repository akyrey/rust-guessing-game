use std::io;
// This is a trait
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Use a local to the current thread of execution generator seeded by os
    // Range is inclusive of the lower bound but exclusive of the upper bound (same as 1..=100)
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");

    // Rust variables are declared using "let" and are immutable by default. Using "mut" will make the mutable
    // new is an "associated function" implemented for type String
    let mut guess = String::new();

    // Returns io::Stdin
    io::stdin()
        // Read input value and append it to guess variable using a reference (&)
        // References as variables are immutable by default
        // Returns io::Result, an enum with variants Ok and Err
        .read_line(&mut guess)
        // If Result is an Err variant, expect method will cause the program to crash and display the message
        .expect("Failed to read line");

    println!("You guess: {}", guess);
}
