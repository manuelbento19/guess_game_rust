use std::io::{stdin};
use rand::{thread_rng, Rng};

fn main(){
    let secret_number = thread_rng().gen_range(0..7);
    println!("Insert the number between 0 and 7: ");
    let mut input = String::new();
    stdin().read_line(&mut input,).expect("Expected receive a number");
    println!("The secret number is {secret_number}");
    println!("Your input is {}",input);
}