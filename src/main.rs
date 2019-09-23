use std::io;

/// This is a doc comment! It gets documented.
///
/// # Examples
///
/// ```
/// println!("Hello!")
/// ```
pub fn foo() {}

fn main() {
    println!("Welcome to the guessing game!");
    println!("Input guess:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess);

    println!("You guessed {}", guess)
}
