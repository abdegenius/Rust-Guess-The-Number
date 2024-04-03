use rand::Rng;
use std::io;
fn main() {
    let mut rng = rand::thread_rng();
    let n: u64 = rng.gen_range(0..101);
    println!("Welcome to this simple GUESS THE NUMBER game!");
    println!("Please guess a number between 0-100");
    let mut user_moves: u64 = 1;
    let mut guessed: bool = false;
    println!("Number to guess: {}", n);
    while (!guessed) {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to enter a number");
        let mut number: u64 = user_input.trim().parse::<u64>().unwrap();
        if (number > n) {
            println!(
                "You Guessed: {}, that is greater than the expected number",
                number
            )
        } else if (number < n) {
            println!(
                "You Guessed: {}, that is less than the expected number",
                number
            )
        } else {
            guessed = true;
            break;
        }
        user_moves += 1;
    }
    if (guessed) {
        println!(
            "After {} Moves, You guessed: {}, that is the correct number. Good JOB!",
            user_moves, n
        );
    } 
}
