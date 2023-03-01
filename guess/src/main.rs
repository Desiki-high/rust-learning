use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut i = 10;

    while i != 0 {
        i -= 1;
        println!("Please input your guess:");
        let guess = get_input();
        if !is_equal(guess, secret_number) {
            if i != 0 {
                println!("You have the lase {i} change!");
            }
        } else {
            println!("You Win!");
            return;
        }
    }
    println!("You Lose!");
}

fn get_input() -> i32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().expect("Please input a number")
}

fn is_equal(num_1: i32, num_2: i32) -> bool {
    match num_1.cmp(&num_2) {
        Ordering::Equal => return true,
        Ordering::Greater => {
            println!("Too Big!");
            return false;
        }
        Ordering::Less => {
            println!("Too Small!");
            return false;
        }
    }
}
