use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(0..=100);
    println!("the random number is {secret_number}");
    
    println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("failed to read the message");
    
    println!("You guessed: {guess}");

    match  guess.cmp(&secret_number.to_string()){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }
}