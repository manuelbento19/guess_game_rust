use std::{cmp::Ordering, io::stdin};
use rand::{thread_rng, Rng};

fn main(){
    let secret_number = thread_rng().gen_range(0..7);
    loop {
        println!("Insert the number between 0 and 7: ");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Expected receive a number");

        let input:u32 = match input.trim().parse(){
            Ok(value) => value,
            Err(_) => continue
        };
        match input.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }

   
}