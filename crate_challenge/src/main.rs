use std::io;
use rand::prelude::*;

fn main() {
    println!("beginning program");

    guessing_game();

    println!("ending program");
}

fn guessing_game() -> bool {
    let number = thread_rng().gen_range(1..11);

    loop {
        let mut buffer = String::new();
        println!("guess a number");
        io::stdin().read_line(&mut buffer).ok();

        let guess = buffer.trim().parse::<i32>().unwrap();

        if guess == number {
            println!("You guessed the number correctly!");
            return true;
        } else if guess > number {
            println!("Your guess was too high!");
        } else {
            println!("Your guess was too low!");
        }
    }
}
