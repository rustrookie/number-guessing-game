extern crate rand;
use std::io;
use rand::Rng;

fn main(){
    println!("'Guess The Number' Game!");
    println!("In this game you'll be guessing the secret number between a range of numbers, the lower being inclusive while the higher being exclusive. To start provide the lower and higher limits of the range of numbers.");

    let mut _lower_limit = String::new();
    
    println!("Choose the lower limit (number > 0): ");

    io::stdin().read_line(&mut _lower_limit)
    .expect("Failed to read value");

    let _lower_limit: u32 = match _lower_limit.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    let mut _higher_limit = String::new();
    
    println!("Choose the higher limit (number > _lower_limit + 1): ");

    io::stdin().read_line(&mut _higher_limit)
    .expect("Failed to read value");

    let _higher_limit: u32 = match _higher_limit.trim().parse() {
        Ok(num) => num,
        Err(_) => 101,
    };

    let secret_number = rand::thread_rng().gen_range(_lower_limit.._higher_limit);
}