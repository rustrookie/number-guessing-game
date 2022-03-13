extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

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

    let mut _moves_made: u32 = 0;

    loop {
        println!("Input guess: ");

        let mut _guess = String::new();

        io::stdin()
            .read_line(&mut _guess)
            .expect("Failed to read input!!!");

        let _guess: u32 = match _guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed the number: {}", _guess);

        _moves_made += 1;

        match _guess.cmp(&secret_number) {
            Ordering::Less  => println!("Too small!!"),
            Ordering::Greater  => println!("Too big!!"),
            Ordering::Equal  => {
                let total_moves = _higher_limit - _lower_limit;
                let total_moves_float = total_moves as f64;
                let moves_made_float = _moves_made as f64;
                let efficiency: f64 = ((total_moves_float - moves_made_float) / total_moves_float) * 100.00;
                println!("YOU HAVE WON!! \n Guesses Made: {}\n Possible Guesses: {}\n Efficiency: {}%" , _moves_made, total_moves, efficiency);
                break;
            }
        }
    }
}