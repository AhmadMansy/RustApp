use std::cmp::Ordering;
use std::io::{stdin};
use rand::Rng;
use colored;
use colored::Colorize;

fn main() {
    guess_game();
}

fn guess_game(){
    let rand:i32 = rand::thread_rng().gen_range( 1..=100);
    let mut count = 0;
    loop {
        let mut guess = String::new();
        println!("Please enter your guess");
        stdin().read_line(&mut guess)
            .expect("failed to read line");
        count += 1;
        let guess: i32 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_) => continue,
        };

        match guess.cmp(&rand){
            Ordering::Less => { println!("{}","Too small!".red())}
            Ordering::Equal => {
                println!("{}",format!("you win in [{}] attempts", count).green());
                break;
            }
            Ordering::Greater => {println!("{}", "Too big!".red())}
        }
    }
}