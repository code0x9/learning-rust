extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::random;

fn get_guess() -> u8 {
    loop {
        println!("Input guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read from stdin");

        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(e) => println!("Could not understand input: {}", e),
        }
    }
}

fn handle_guess(guess: u8, correct: u8) -> bool {
    return match guess.cmp(&correct) {
        Ordering::Less => {
            println!("Too low");
            false
        }
        Ordering::Greater => {
            println!("Too high");
            false
        }
        Ordering::Equal => {
            println!("You got it!");
            true
        }
    };
}


fn main() {
    println!("Welcome to the guessing game!");
    let correct = random::<u8>();
//    println!("correct: {}", correct);

    loop {
        let guess = get_guess();
        if handle_guess(guess, correct) {
            break;
        }
    }
}
